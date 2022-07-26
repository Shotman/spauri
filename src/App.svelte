<script lang="ts">
	import LogoTao from './assets/logo_tao.svg';
	import LogoWry from './assets/logo_wry.svg';
</script>

<main>
	<img src={LogoTao} alt="Tao" />
	<img src={LogoWry} alt="Wry" />
</main>

<style lang="scss">
	main {
		width: 100vw;
		height: 100vh;

		display: grid;
		place-content: center;

		img {
			width: Clamp(200px, 50vh, 500px);
		}
	}
</style>
