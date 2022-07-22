<script lang="ts">
    import { connectSigner, disconnect, signer, Signer } from "../util/store";
    let s: Signer | false = false;
    signer.subscribe((x) => (s = x));
</script>

<div>
    <p>Connect or Disconnect Signer</p>
    {#if !s}
        <button on:click={connectSigner}>Connect</button>
    {:else}
        <button
            on:click={disconnect}>Disconnect</button
        >
    {/if}
</div>
