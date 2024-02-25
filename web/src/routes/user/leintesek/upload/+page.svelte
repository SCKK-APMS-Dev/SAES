<script lang="ts">
	import Upload from '$lib/upload.svelte';
	export let data;
</script>

<Upload
	adat={data}
	title="Leintés"
	underhood="leintes"
	warning="FONTOS! Bármennyit feltölthetsz csak a sorrend legyen meg! (Leintésenként 2 kép egymás után
kiválasztva, a fájlkezelőben visszafelé kell kijelölni, hogy a sorrent megmaradjon)"
/>
