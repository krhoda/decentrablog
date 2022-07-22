<script lang="ts">
	import { Router, Route } from "svelte-navigator";
	import { Header, Read, SignIn, Write } from "./components";
</script>

<Router>
	<Header />
	<Route path="/">
		<Read />
	</Route>
	<Route path="sign-in">
		<SignIn />
	</Route>
	<Route path="write">
		<Write />
	</Route>
</Router>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>
