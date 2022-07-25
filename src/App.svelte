<script lang="ts">
	import { Content } from 'carbon-components-svelte';
	import { Sidebar } from './components/sidebar';
	import { Player } from './components/player';
	import { Route, router } from 'tinro';

	router.mode.hash();
</script>

<Sidebar id="main-sidebar" />

<Content id="main-content">
	<Route path="/">
		<h1>Home</h1>
	</Route>
</Content>

<Player id="main-player" />

