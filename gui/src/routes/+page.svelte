<script async script lang="ts">
	import { app, window } from '@tauri-apps/api';
	import { invoke } from '@tauri-apps/api/core';
	import { currentMonitor } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	let name = '';
	let greetMsg = '';

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke('greet', { name });
	}
	async function checkWin(wind: window.Window | null, prevGame: boolean) {
		let screen = await currentMonitor();
		console.log(screen?.size);
		let win: string = await invoke('check_win');
		let win_buf = win.split('\\\\');
		if (win_buf[win_buf.length - 1] === 'gta_sa.exe"' || prevGame) {
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
			checkWin(wind, prevGame);
		}, 5000);
	}
	onMount(async () => {
		let main = await window.Window.getByLabel('main');
		checkWin(main, true);
	});
</script>

<div class="flex h-screen">
	<img class="m-auto ml-2 h-[40px]" src="/icon.png" alt="" />
</div>
<p class="absolute left-[55px] top-0 text-lg uppercase text-white">SCKK GUI</p>
<p class="absolute bottom-0 left-[55px] text-lg text-gray-400">Betöltés</p>
<!-- <p class="absolute bottom-0 left-[55px] text-lg text-green-400">Szolgálatban</p> -->
<!-- <p class="absolute bottom-0 left-[55px] text-lg text-red-600">Nem aktív</p> -->
