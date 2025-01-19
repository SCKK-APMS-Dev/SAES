<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	let page = $state('noconfig');
	listen<string>('setmainpage', (ev) => {
		page = ev.payload;
	});
</script>

<div class="bg-gray-950 h-screen w-screen text-white text-center pointer-events-none select-none">
	{#if page === 'nothing'}
		<div class="flex justify-center items-center h-full">
			<img src="/favicon.png" alt="" />
		</div>
	{/if}
	{#if page === 'noconfig'}
		<div class="bg-slate-500 w-screen py-1">
			<h1 class="font-bold text-3xl">Konfigurációs mágus</h1>

			<h2>Pár kattintás után lehetőséged nyílik eme csoda használatára.</h2>
		</div>
	{/if}
</div>
