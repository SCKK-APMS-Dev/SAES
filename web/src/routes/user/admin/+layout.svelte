<nav class="bg-blue-700 flex text-white justify-around items-center flex-col md:flex-row">
	<div class="flex items-center gap-2">
		<a href="info" class="font-bold text-3xl drop-shadow-xl">Műszakvezetés</a>
	</div>
	<div
		class="flex gap-2 text-xl flex-col text-center md:flex-row md:gap-10 child:w-screen child:md:w-auto child:border-y-2 child:md:border-none child:drop-shadow-xl"
	>
		<a href="/user/admin" class="hover:font-bold duration-200 transition-all">Jelenlegi hét</a>
		<a href="/user/admin/prev" class="hover:font-bold duration-200 transition-all">Előző hét</a>
		<a href="/user/admin/potlekok" class="hover:font-bold duration-200 transition-all">Pótlékok</a>
		<a href="/user/admin/leintesek" class="hover:font-bold duration-200 transition-all">Leintések</a
		>
		<a href="/user/admin/szamlak" class="hover:font-bold duration-200 transition-all"
			>Szereltetési számlák</a
		>
	</div>
</nav>
<slot />
