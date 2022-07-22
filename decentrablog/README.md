# Decentrablog a Hackathon Project
Using a vendored version of Rebase, this project is a PoC of a VC based blog.

Once complete, will be locally runable by running the following commands from the root of the repo.

Start the witness
```
$ cd rebase/demo/witness
$ wrangler dev
```

Then in another terminal
```
$ cd decentrablog
$ npm i && npm run dev
```

This will allow the two pieces of the demo to communicate locally.