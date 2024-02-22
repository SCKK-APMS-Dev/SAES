<script lang="ts">
	export let data;
</script>

<div class="h-screen flex">
	<div class="m-auto text-center text-white">
		{#if data?.error}
			<h1 class="text-3xl text-red-600 font-bold">{data.error.msg}</h1>
		{/if}
		{#if data?.admin}
			<h1>Tagok</h1>
			<table class="table-auto p-10">
				<thead class="bg-red-700">
					<tr class="child:p-2">
						<th>ID</th>
						<th>Discord ID</th>
						<th>IG Név</th>
						<th>Rang</th>
					</tr>
				</thead>
				<tbody>
					{#each data?.admin.tagok as tag}
						<tr>
							<td>{tag.id}</td>
							<td>{tag.discordid}</td>
							<td>{tag.name}</td>
							<td>{tag.role}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	</div>
</div>
