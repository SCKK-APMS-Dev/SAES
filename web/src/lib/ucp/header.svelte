<script lang="ts">
	import { snow } from '$lib/api';
	import { pages } from './public';
	import { page as statepage } from '$app/state';
	import { Tooltip } from 'flowbite-svelte';

	interface Props {
		tip: any;
		isAdmin?: boolean;
		faction: string;
		data: {
			taxi?: any;
			tow?: any;
			admin?: boolean;
			perms?: string[];
		};
	}

	let { tip, isAdmin = false, faction = 'SCKK', data }: Props = $props();

	let multifact =
		(data.perms?.includes('saes.ucp.taxi') && data.perms.includes('saes.ucp.tow')) || data.admin;
	let pagesz = pages(faction);
</script>

<header class="z-20">
	<div class="relative z-20 border-b bg-white dark:bg-gray-700 dark:text-white">
		<div class="mx-0 px-0 xl:container lg:mx-auto lg:py-4">
			<div class={`${!multifact ? 'group' : ''} flex items-center justify-between gap-2`}>
				<div class="flex items-center justify-center gap-3">
					<a
						class="group relative z-20 flex items-center"
						data-sveltekit-reload
						href={multifact ? '?clear_faction=true' : '/ucp'}
					>
						<img
							src="/favicon.png"
							class={`pointer-events-none ml-5 rounded-full border-2 border-solid drop-shadow-xl transition-colors duration-200 ${faction === 'TOW' ? 'group-hover:border-tow' : 'group-hover:border-taxi'}`}
							width="40"
							height="40"
							alt="SCKK Logó"
						/>
						{#if snow}
							<img
								src="/santa.svg"
								class="absolute bottom-2 left-3.5 w-14 -rotate-[24deg]"
								alt=""
							/>
						{/if}
					</a>
					{#if multifact}
						<Tooltip>Részlegváltás</Tooltip>
					{/if}
					<a
						href="/ucp"
						class={`z-20 text-3xl font-bold drop-shadow-xl transition-colors duration-200 ${
							faction === 'TOW'
								? multifact
									? 'hover:text-tow'
									: 'group-hover:text-tow'
								: multifact
									? 'hover:text-taxi'
									: 'group-hover:text-taxi'
						}`}
					>
						{tip}
					</a>
				</div>

				<div class="flex items-center justify-end border-l lg:border-l-0">
					<input type="checkbox" name="hamburger" id="hamburger" class="peer opacity-0" hidden />
					<label
						for="hamburger"
						class="peer-checked:hamburger relative z-20 mr-6 block cursor-pointer p-6 lg:hidden"
					>
						<div
							aria-hidden="true"
							class="m-auto h-0.5 w-6 rounded bg-white transition duration-300"
						></div>
						<div
							aria-hidden="true"
							class="m-auto mt-2 h-0.5 w-6 rounded bg-white transition duration-300"
						></div>
					</label>

					<div
						class="fixed inset-0 w-[calc(100%-4.5rem)] translate-x-[-100%] border-r bg-white shadow-xl transition duration-300 peer-checked:translate-x-0 lg:static lg:w-auto lg:translate-x-0 lg:border-r-0 lg:shadow-none dark:bg-gray-700"
					>
						<div class="flex h-full flex-col justify-between lg:flex-row lg:items-center">
							<ul
								class="items-center pt-32 text-center text-gray-700 md:space-y-8 lg:flex lg:space-x-3 lg:space-y-0 lg:px-2 lg:pt-0 xl:space-x-12 xl:px-12"
							>
								{#each pagesz as page}
									<li>
										<a
											href={page.url}
											class={`${faction === 'TOW' ? 'before:bg-tow' : 'before:bg-taxi'} group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left ${statepage.url.pathname === page.url ? 'before:scale-x-100' : 'hover:before:scale-x-100'}`}
										>
											<span class="relative text-black dark:text-white">{page.display}</span>
										</a>
									</li>
								{/each}
							</ul>

							<div
								class="border-t px-6 py-8 md:px-12 md:py-16 lg:border-l lg:border-t-0 lg:py-0 lg:pl-6 lg:pr-0"
							>
								{#if isAdmin}
									<div class="flex items-center gap-3">
										<a
											href="/ucp/sm"
											class={`${faction === 'TOW' ? 'from-tow via-blue-600 to-emerald-400' : 'from-taxi via-amber-600 to-red-500'} hover:bg-pos-100 bg-size-200 bg-pos-0 block rounded-full bg-gradient-to-r px-6 py-3 text-center font-bold text-white drop-shadow-lg transition-all duration-500`}
										>
											Műszakvezetés
											{#if snow}
												<span class="icon-[fluent-emoji--sled] absolute bottom-11 right-8 h-6 w-6"
												></span>
											{/if}
										</a>
									</div>
								{/if}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
	<h2 class="z-20 bg-gradient-to-r from-rose-600 to-amber-600 py-1 text-center text-xl text-white">
		Nem vagy biztos valamiben? Nézd meg a <a href="/ucp/segedlet" class="text-taxi z-20 font-bold"
			>segédletet</a
		>!
	</h2>
</header>
