<script lang="ts">
    import { connectSigner, signer, Signer } from "../util/store";
    let s: Signer | false = false;
    signer.subscribe((x) => (s = x));
</script>

<div>
    <p>This is Sign In.</p>
    {#if !s}
        <button on:click={connectSigner}>Connect</button>
    {:else}
        <button
            on:click={() => {
                alert("UNIMPLEMENTED!");
            }}>Disconnect</button
        >
    {/if}
</div>
