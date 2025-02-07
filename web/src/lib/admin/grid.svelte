<script lang="ts">
	let {
		data,
		title,
		items
	}: {
		data: { faction?: string };
		title?: string;
		items: {
			title: string;
			description: string;
			border: string;
			background: string;
			faction: string[];
			href: string;
		}[];
	} = $props();
</script>

<div class="flex items-center text-center text-white">
	<div class="m-auto">
		<h1 class="font-itim mb-5 mt-5 text-4xl font-bold">{title}</h1>
		<div class="flex w-screen items-center justify-center gap-5">
			{#each items as item}
				{#if item.faction.includes(data.faction!)}
					<a
						href={item.href}
						class={`${item.border} ${item.background} w-1/2 rounded-3xl border-4 p-5 transition-colors duration-500 lg:w-1/4`}
					>
						<h1 class="text-2xl font-bold">{item.title}</h1>
						<h2 class="font-itim">
							{item.description}
						</h2>
					</a>
				{/if}
			{/each}
		</div>
	</div>
</div>
