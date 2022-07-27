<script lang="ts">
	import LogoTao from './assets/logo_tao.svg';
	import LogoWry from './assets/logo_wry.svg';
</script>

<main>
	<div class="img">
		<img src={LogoTao} alt="Tao" />
		<span>-</span>
		<img src={LogoWry} alt="Wry" />
	</div>

	<div class="header">
		<h1>Spauri</h1>
	</div>
</main>

<style lang="scss">
	main {
		width: 100vw;
		height: 100vh;

		display: grid;
		place-content: center;

		background-color: #1a1a1a;

		.img {
			display: flex;
			flex-direction: row;

			justify-content: center;
			align-items: center;

			span {
				color: #eee;
				font-family: "Fira Code";

				margin: 0 20px;

				font-size: 5rem;
			}

			img {
				width: Clamp(200px, 50vh, 500px);
			}
		}

		.header {
			width: 100vw;
			height: 150px;

			display: grid;
			place-content: center;

			h1 {
				color: #eee;
				font-family: "Fira Code";

				font-size: 5rem;
			}
		}
	}
</style>
