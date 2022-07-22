<script lang="ts">
    import { Link } from "svelte-navigator";
    import { signer, Signer, KeyType, witnessUrl, posts } from "../util/store";
    let s: false | Signer = false;
    signer.subscribe((x) => (s = x));

    let p: Array<Object> = [];
    posts.subscribe((x) => (p = x));

    $: title = "";
    $: body = "";

    const makeKeyType = (): KeyType => {
        if (!s) {
            throw new Error("Please connect your wallet");
        }

        return {
            pkh: {
                eip155: {
                    address: s.id(),
                    chain_id: "1",
                },
            },
        };
    };

    const post = async () => {
        try {
            let opts = {
                basic_post: {
                    title,
                    body,
                    key_type: makeKeyType(),
                },
            };

            let res = await fetch(`${witnessUrl}/statement`, {
                method: "POST",
                body: JSON.stringify({ opts }),
                headers: {
                    "Content-Type": "application/json",
                },
            });

            let j = await res.json();

            let { statement } = j;

            if (s) {
                let signature = await s.sign(statement);
                let proof = {
                    basic_post: {
                        signature,
                        statement_opts: {
                            title: opts.basic_post.title,
                            body: opts.basic_post.body,
                            key_type: opts.basic_post.key_type,
                        },
                    },
                };

                res = await fetch(`${witnessUrl}/witness?type=basic_post`, {
                    method: "POST",
                    body: JSON.stringify({ proof }),
                    headers: {
                        "Content-Type": "application/json",
                    },
                });

                j = await res.json();

                let { jwt } = j;
                console.log(jwt);

                // let credential = fmt(jwt)
                // let nextPosts = p.map((x) => x);
                // nextPosts.push(credential);
                // posts.set(nextPosts);

                title = "";
                body = "";
            }
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
    <div>Please <Link to="/sign-in">sign in</Link></div>
{/if}
