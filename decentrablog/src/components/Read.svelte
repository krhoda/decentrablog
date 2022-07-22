<script lang="ts">
    import { posts } from "../util/store";
    interface Post {
        author: String;
        title: String;
        body: String;
        posted: Date;
    }

    const credentialToPost = (o: Object): Post | false => {
        return false;
    };

    let fmtPosts: Array<Post> = [];
    posts.subscribe((x) => {
        const temp = x.map(credentialToPost);
        const nextPosts: Array<Post> = [];
        temp.forEach((maybePost) => {
            if (maybePost) {
                nextPosts.push(maybePost);
            }
        });

        nextPosts.sort((a, b) => {
            return b[0].posted.getTime() - a[0].posted.getTime();
        });

        fmtPosts = nextPosts;
    });
</script>

{#if fmtPosts.length <= 0}
    <p>No Posts Generated Yet</p>
{:else}
    {#each fmtPosts as post}
        <div>
            <p>Title: {post.title}</p>
            <p>Author: {post.author}</p>
            <p>Body: {post.body}</p>
        </div>
    {/each}
{/if}
