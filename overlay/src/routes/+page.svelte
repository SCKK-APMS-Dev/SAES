<script async script lang="ts">
	import { app, window } from '@tauri-apps/api';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	let name = '';
	let greetMsg = '';

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke('greet', { name });
	}
	async function checkWin(wind: window.Window | null) {
		let win: string = await invoke('check_win');
		let win_buf = win.split('\\\\');
		if (
			win_buf[win_buf.length - 1] === 'gta_sa.exe"' ||
			win_buf[win_buf.length - 1] === 'sckkextra-overlay.exe"'
		) {
			if (!(await wind?.isVisible())) {
				wind?.show();
			}
		} else {
			if (await wind?.isVisible()) {
				console.log(win_buf[win_buf.length - 1]);
				wind?.hide();
			}
		}
		setTimeout(() => {
			checkWin(wind);
		}, 2000);
	}
	onMount(async () => {
		let main = await window.Window.getByLabel('main');
		checkWin(main);
	});
</script>

<div class="flex h-screen">
	<img class="m-auto" src="/icon.png" alt="" on:click={() => greet()} />
</div>
