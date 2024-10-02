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
	async function checkWin(wind: window.Window | null) {
		let screen = await currentMonitor();
		console.log(screen?.size);
		let win: string = await invoke('check_win');
		let win_buf = win.split('\\\\');
		// if (
		// 	win_buf[win_buf.length - 1] === 'gta_sa.exe"' ||
		// 	win_buf[win_buf.length - 1] === 'sckkextra-overlay.exe"'
		// ) {
		// 	if (!(await wind?.isVisible())) {
		// 		wind?.show();
		// 	}
		// } else {
		// 	if (await wind?.isVisible()) {
		// 		console.log(win_buf[win_buf.length - 1]);
		// 		wind?.hide();
		// 	}
		// }
		setTimeout(() => {
			checkWin(wind);
		}, 5000);
	}
	onMount(async () => {
		let main = await window.Window.getByLabel('main');
		checkWin(main);
	});
</script>

<div class="flex h-screen">
	<img class="m-auto ml-2 h-[40px]" src="/icon.png" alt="" />
</div>
<p class="absolute left-[55px] top-0 text-xl font-bold uppercase text-white">SCKK Átfedés</p>
<p class="absolute bottom-0 left-[55px] text-lg font-bold text-red-600">Nem aktív</p>
