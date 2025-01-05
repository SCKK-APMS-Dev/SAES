<script lang="ts">
	interface Props {
		title?: string;
	}

	let { title = 'SCKK' }: Props = $props();
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>
