<script lang="ts">
    import { Link } from "svelte-navigator";
    import { signer, Signer } from "../util/store";

    let _signer: false | Signer = false;
    signer.subscribe((x) => (_signer = x));
</script>

<div class="border-dark-800">
    {#if _signer}
        <p>
            Signed in as {_signer.ens?.name ? _signer.ens.name : _signer.id()}
        </p>
    {:else}
        <p>Not signed in</p>
    {/if}
    <p><Link to="sign-in">{_signer ? "Sign Out" : "Sign In"}</Link></p>
    <p><Link to="/">View Blog Posts</Link></p>
    <p><Link to="write">Create New Blog Post</Link></p>
</div>
