<script lang="ts">
    import { onMount } from "svelte";
    import { posts } from "../util/store";
    interface Post {
        author: String;
        title: String;
        body: String;
        posted: Date;
    }

    const encode = (c): string => {
        return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
    };

    const parseJWT = (jwt_str: string): any => {
        const v = jwt_str.split(".");

        if (v.length !== 3) {
            throw new Error("Invalid JWT format");
        }

        const u = v[1];
        const b64 = u.replace(/-/g, "+").replace(/_/g, "/");
        const encoded = atob(b64).split("").map(encode).join("");
        const json_str = decodeURIComponent(encoded);

        return JSON.parse(json_str);
    };

    const credentialToPost = (jwt: string): Post => {
        let o = parseJWT(jwt);
        let p: Post = {
            author: o?.sub,
            title: o?.vc?.credentialSubject?.title,
            body: o?.vc?.credentialSubject?.body,
            posted: new Date(o?.vc?.issuanceDate),
        };

        return p;
    };

    let _posts: Array<string> = [];
    posts.subscribe((x) => (_posts = x));
    let fmtPosts: Array<Post> = [];

    onMount(() => {
        fmtPosts = _posts.map(credentialToPost).sort((a, b) => {
            return b.posted - a.posted;
        });
    });
</script>

{#if fmtPosts.length <= 0}
    <p>No Posts Generated Yet</p>
{:else}
    {#each fmtPosts as post, i}
        <div>
            <p>Title: {post.title}</p>
            <p>Body: {post.body}</p>
            <p>Author: {post.author}</p>
            <p>Posted: {post.posted}</p>
        </div>
    {/each}
{/if}
