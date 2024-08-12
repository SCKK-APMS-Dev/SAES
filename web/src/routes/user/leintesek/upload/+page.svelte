<script lang="ts">
	import Upload from '$lib/user/upload.svelte';

	export let data;
</script>

<Upload
	{data}
	display="Leintés"
	tipus="leintés"
	warning="Egy leintés 2 képből áll, magyarul a 2 többszörösét fogadja el a rendszer! (Egyszerre több leintést is fel lehet tölteni)"
/>
