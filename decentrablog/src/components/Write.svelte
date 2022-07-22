<script lang="ts">
    import { Link } from "svelte-navigator";
    import {
        delimitor,
        signer,
        Signer,
        witnessUrl,
        posts,
    } from "../util/store";
    let s: false | Signer = false;
    signer.subscribe((x) => (s = x));

    let p: Array<Object> = [];
    posts.subscribe((x) => (p = x));

    $: title = "";
    $: body = "";

    const post = async () => {
        try {
            let b = {
                basic_post: {
                    title,
                    body,
                    delimitor,
                },
            };

            let c = await fetch(witnessUrl, {
                method: "POST",
                body: JSON.stringify(b),
                headers: {
                    "Content-Type": "application/json",
                },
            });

            let nextPosts = p.map((x) => x);
            nextPosts.push(c);
            posts.set(nextPosts);

            title = "";
            body = "";
        } catch (e) {
            alert(e);
        }
    };
</script>

{#if s}
    <div>
        <div>
            <label for="title">Title</label>
            <input type="text" name="title" bind:value={title} />
        </div>
        <div>
            <label for="body">Body</label>
            <input type="text" name="body" bind:value={body} />
        </div>

        <button on:click={post}>Submit</button>
    </div>
{:else}
    <div>Please <Link to="sign-in">sign in</Link></div>
{/if}
