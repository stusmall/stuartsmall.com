+++
title = "Automating online recipe cataloge"
template = "page.html"
date = 2023-01-22
+++

Sometimes a recipe I love disappears off the internet. Maybe the blog shuts down, or the author tries to update but
ruins it. Overtime I tweak and change around recipes. I used to keep my recipes in a series of text files on my laptop,
but that wasn't great. Without an organizational method it ended up getting messy quickly. In addition to this sometimes
when I'm at the grocery store I want to double-check the quantity on some ingredients. It had been on my back burner for
years to find a better method, but I never got around to it.

Until one day, my spouse and I remembered an absurd recipe from years ago. It was the one that finally won me over on
instant pots. It was a jambalaya recipe where you threw every raw ingredient together and hit start. Raw chicken, raw
rice, everything. At the time it kind of grossed me out and unsettled me, but I gave it a go anyway. The meal came out
perfect. It was counter what I was used to and came out fantastic for the tiny amount of effort. We were joking around
about it and wanted to try it again. Unfortunately I hadn't written it down. I looked back and found an old email where
I linked to it but sadly the link was broken just redirected to a search page.

Luckily, my dad had a copy of the recipe in his notes, but I knew I needed a better system. The point of the post is
to walk through that journey. Like all software projects it has some dead ends and experiments. If you don't want to
hear about that and just want to see how to do your own, skip to the last section for steps and links to code to borrow.

## Enter Cooklang

A while back a friend sent me a link to Cooklang. I thought it looked cool but at the time I needed another project
like I needed a hole in my head. Cooklang is a Markdown language used to describe recipes. When you annotate your
recipe with it then it can produce plaintext recipe cards, shopping lists, it can be imported into a mobile app, etc. It
is there to serve as a common, machine-readable format to store your recipes in. If I'm doing to go through the effort
of copying all these recipes, it will at least be nice to store it in common format.

Cooklang isn't the first project to attempt this. I did a brief search and poked at some of the others. There is
RecipeML, a recipe JSON schema type, Open Recipe Format, and countless others. After I looked at a couple I decided it
honestly doesn't matter. As long as the data format is decently well-structured and provides some support libraries, if
I end up hating it I can just migrate my recipes away.

## An introduction to a Cooklang file

Cooklang is a very simple markup language. There are only a few things you need before you can
dive right in.

At the top of the file you can create a section for metadata. This can include things like a source, data created,
tags, a description, etc. Some of these values might have special meanings to applications, but in general you can
treat it like an arbitrary key value store. An example from the docs:

```
>> source: https://www.gimmesomeoven.com/baked-potato/
>> time required: 1.5 hours
>> course: dinner
```

Next you will add the steps. Each line is a step in the recipe. Text in a step can be annotated as a timing,
ingredient or a piece of cookware. In each of these annotations a % is used to separate quantity from the unit. For
example, a recipe to cook white rice in an instant pot:

```
Put the @rice{2%cups} and @water{3%cups} in the #instant pot{}.

Cook for ~{4%minutes} and then natural release for ~{10%minutes}
```

With this you have the basics and can probably write out most recipes. There are some more techniques you should see
the docs for. Like how to build [shopping lists](https://cooklang.org/docs/spec/#the-shopping-list-specification)
or [optional language extensions](https://github.com/cooklang/cooklang-rs/blob/main/extensions.md) for things like
cross recipe references or optional ingredients.

I'm someone who learns best by playing around. If you are the same way there is
also [a great online playground where you can experiment with the
syntax.](https://biowaffeln.github.io/cooklang-parser/)

## Setting up the tools

The first thing I needed to do was install some tools to try it out. Because I absolutely refuse to make my life simple
in any way I've recently been experimenting with NixOS. So far almost any software I've needed has been in nixpkgs. When
I've needed to write a derivation before it's pretty small and simple. Unfortunately neither of the two most
popular Cooklang CLI tools were in nixpkgs yet, so I had to do it myself. This isn't a post about nix, so I won't go
too deep into the process or my thoughts about that project. I will say I found some good advice in a GitHub issue,
[slammed something messy together, posted my results](https://github.com/NixOS/nixpkgs/issues/260025#issuecomment-1872639154),
and it looks
like [someone else is picking up cleaning it up and putting in the effort to get it upstreamed](https://github.com/NixOS/nixpkgs/pull/280610).
I always love to see this out of open source software. A few different people across the global putting in time where
they can, helping each other out, and then we all end up better off.

## First steps

I have one recipe that is pretty central to keeping my spouse around. Whenever they leave on a trip I always make them
biscuits to take with them. It's great for them to have a comforting piece of home while out on the road. It's great
for me to know they will actually be eating at least some real food. This was the first recipe I transcribed.

```
Preheat oven to 425F.

Cube @butter{8%tbsp}

In a #large mixing bowl{} combine @flour{2%cup}, @sugar{1%tsp}, @baking powder{1%tbsp}, @salt{1%tsp}.

Cut in butter with #potato masher{} until it looks like cornmeal.

Slowly add @milk{1%cup} and gently knead.  Add milk until desired consistency.

Flatten out to desired thickness in mixing bowl.

Cut out biscuits and refold as need.

Bake in #cast iron skillet{} for ~{12%minutes} or until golden brown
```

Now with this I could easily play around with what the official tooling offered. One nice feature that the
cook-cli has is a full-featured built-in webapp.

![The UI of the built in cook-cli webapp](../cookcli-serve.webp)

It tracks ingredient lists, needed tools, etc. It even has the ability to dynamically built out shopping lists from
selected recipes making meal planning easier. In seeing what could be done with the tool, I was sold.

While I saw that would what could be done, I didn't want to use the current application.

## An aside on static site generators

I'm a huge fan of static site generators. If you aren't familiar with the idea, it is a program that generates static
HTML usually by running some markdown through a some HTML templates. I've used them in the past for an art site, API
docs among other things. Anytime I have a need for a site and I can scratch the itch with a static site generator, I do
it.

Huge swaths of security issues are just gone when using one. You can't hack a service that doesn't exist. Tons of
performance and scalability concerns are gone too. Modern webservers handle serving static content exceptional well.
They will easily scale far past to way past whatever demand people have to read my scribblings. There are tons of great
services out there that will host your static site for free in hopes that you will opt into some of their paid addons.

I like [netlify](http://netlify.com/) for hosting. On the generation side I've used a few different tools. I started out
with Jekyll, but lately I've been using [zola](https://www.getzola.org/). Zola won me over because a lot of the image
manipulation features I got from Jekyll plugins are baked right in. I was already using tera templates at
work for generating nginx configs so there wasn't much I needed to learn. It was easy pick up, worked great and
whenever I needed to send change upstream I found the codebase to be easy to work with and high quality. I'm not too
picky about the static code generator itself though. As long as it can convert markdown to HTML, then I'm a happy man.

I had no interest in trying to host, maintain and secure an instance of the cook-cli webapp. I don't even know if the
original author ever meant for it to be public internet facing. If I could find a way to build static HTML that is what
I was going to do. My first though was to either separate out what they already had or send patches upstream to add a
static site generation feature. Before I dove in too deep I reached out on the official Discord. Someone there pointed
out that the cook-cli had an option to convert cook files to markdown and I should try that. It was such an "oh duh"
idea. It let me lean more on upstream tools, keep Cooklang specific logic separate from my site generation and keep it
all very decoupled. So ideally the pipeline would look like:

Cooklang files -> build with cook-cli -> markdown files -> zola build -> HTML -> deploy to netlify

It looked straight forward enough and it was. It took no time at all to get everything plugged together and working
locally. Most of that was spent on updating an old set of templates for a site I wrote years ago but never really did
anything with.

![The UI of the built in cook-cli webapp](../cooklang-localscreenshot.webp)

There were a couple useful Cooklang specific settings I needed. The first was ignoring the cook source in zola's
Config.toml with "ignored_content = [ "*.cook"]". Once we build markdown out of these files, we don't need them again.
No reason to make them available to serve. In a related but opposite note, I didn't want the produced Markdown checked
in to git. Anytime a project includes both the source and the output of a build process eventually they will get out of
sync and cause confusion. This led me to a cool feature of .gitignore that I didn't know about. You can use "!" to
exclude something from the pattern on the previous line. So in my project I have:

```gitignore
/content/recipes/*.md
!/content/recipes/_index.md
```

This says "ignore all markdown files in the recipes directory EXCEPT _index.md".

For a quick pass it worked great. You can see there are a couple ood things. For example, the units are in italics. This
is an artifact of the cooklang-to-md crate adding * around them. Also, when the CLI processes values in the
Cooklang metadata values it would apply special logic to some keys. One of these special keys was "description" but
unfortunately this is also a special key for zola. This limited my ability to control the layout of the recipe
description and also prevented me from providing a quick blurb on the index page.

These were just minor inconveniences and didn't really take away from the short term usability. I reached out on the
Discord again for ideas and decided to move on to getting this deployed.

## Updating CI

Typically,
with [Netlify you just link your GitHub repo with them and their automation takes it from there](https://www.getzola.org/documentation/deployment/netlify/#automatic-deploys).
When set up this way whenever there is a commit the source will be picked up by Netlify, they will build it and the
resulting HTML will be hosted.

Unfortunately now I have this extra build step that requires more tooling. It is probably possible to get nix
installed and working inside Netlify's build runners. I didn't dig too deeply into that and decided to just go with
what I was comfortable with, GitHub Actions. The steps I wanted were:

1. Install nix
2. Use nix to install all other dependencies
3. Use cook-cli to create a markdown file for each cook file.
4. Build the site.
5. Deploy it.

So for each of these, except 3, there is a pretty straight forward prebuilt action on the marketplace to handle it for
me. The only problem is that the Netlify publish action is a community maintained one. So allow me a moment to vent
about my one
big complaint with GitHub Actions. Currently, the versioning scheme of actions is based on git tags and semantic
versioning. The big problem with this is that it isn't always clear exactly what versions is in use, you can't lock to a
version and, worst of all, tags are mutable!  Meaning if you try to pin to specific tag that you have audited, a
malicious actor could just rewrite the tags to sneak out a new version!  [I found this blog post useful in talking about
the issue](https://blog.rafaelgss.dev/why-you-should-pin-actions-by-commit-hash). Securing your CI pipeline in an
underappreciated step in building a secure product. No matter what your project is and what its threat model is, the CI
pipeline is an attractive target and can have catastrophic results if compromised.

In this case think about what a malicious actor could do with access to our CI pipeline. They could add malware to our
nix config, they could edit the HTML after it is built, or most likely, they could harvest the Netlify API key. I don't
think my silly cooking blog is going to face any kind of targeted attack, but a broad campaign for collecting API keys
for a later scam is completely realistic. Phishing the maintainer of an action like this could be very lucrative.
Currently, the best solution for this is referencing actions by git commit rather than tag and including a comment of
the version.

After vetting the actions I wanted to use I came up with this workflow file:

```yaml
name: Build and Deploy to Netlify
on:
  push:
    branches:
      - main
  pull_request:
concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.ref }}
  cancel-in-progress: true
jobs:
  build:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4.1.1
      - uses: cachix/install-nix-action@7ac1ec25491415c381d9b62f0657c7a028df52a7 # v24
        with:
          nix_path: nixpkgs=channel:nixos-23.11
      - name: Build nix environment
        run: nix-build
      - name: Build cook files
        run: nix-shell --command './scripts/build_cookfiles.sh'
      - name: Build site
        run: nix-shell --command 'zola build'
      - name: Deploy to Netlify
        uses: nwtgck/actions-netlify@7a92f00dde8c92a5a9e8385ec2919775f7647352 #v2.10
        with:
          publish-dir: './public'
          production-branch: main
          github-token: ${{ secrets.GITHUB_TOKEN }}
          deploy-message: "Deploy from GitHub Actions"
          enable-pull-request-comment: true
          enable-commit-comment: false
          overwrites-pull-request-comment: true
        env:
          NETLIFY_AUTH_TOKEN: ${{ secrets.NETLIFY_AUTH_TOKEN }}
          NETLIFY_SITE_ID: ${{ secrets.NETLIFY_SITE_ID }}
        timeout-minutes: 1
```

Now we are live!  On any PR it will build and deploy a preview version of the site. This lets you test and verify it
before merging. Then once it is on main it will build and deploy it to the production site. No fuss, no muss, no
human interaction outside the PR. The way a deploy should be.

## Customizing Markdown

So I mentioned earlier there were a few things I didn't like about the produced markdown. They were minor issues, not
enough to stop me from getting the site live but still annoying. After I mentioned this the Discord one of the
contributors was kind enough to add some customization options to one of the crates to make my life easier.
Unfortunately at this point I was noticing more things I wanted to change and some were very specific to my project and
not relevant to the upstream. For example, I wanted to change how the front matter was produced to use Cooklang tags
as a [zola taxonomy](https://www.getzola.org/documentation/content/taxonomies/).

The Cooklang project is great about separating itself out in to individual and easy to use crates published on
crates.io. The parser itself is a [crate](https://crates.io/crates/cooklang),
the [functionality of producing markdown is published on its own](https://crates.io/crates/cooklang-to-md).
All I needed to do was use the upstream parser and pass the output to my own fork of the markdown crate. Since this is
just for
me, [I got a little hacky with it](https://github.com/stusmall/stuartsmall.com/tree/9821b81a13bf74590add4701faae41f1f27ba94b/build-md).
I found the parser quick and easy to work with and the Markdown code clear and easy to update. I'm used to writing in
Rust, but for folks who don't know
it [they offer parsers in tons of different languages](https://cooklang.org/docs/for-developers/). This should make it
easy for anyone to
do the same.

![Final version of a rendered recipe](../cooklang-site-final.webp)

I appreciated that cook-cli gave me a quick path to get start and experiment. It let me see what an application could
be, gave me a little help getting started, before I finally moved on to building my own tool.

## Next steps

This isn't project complete, but I've already found it useful. I recently had friends over and someone asked for my
cornbread recipe. I was able to just quickly just send them a link to my site. I'm slowly going to work through my
recipes as I cook and convert them to Cooklang. I still have plans for development though.

The big change I have in mind is building out nutritional data for each recipe. I've been doing reading on data source
and planning out how I think this could be implemented. Since the recipes and measurements are in a machine-readable
format and the USDA provides a very complete database of nutritional facts this looks to be very doable. The big
questions I have are how to make it ergonomic and easy to work with. Stay tuned on that.

## Steps to follow along at home

If I were to start again from scratch knowing what I know now, these are the steps I would take:

1. Build templates and stylesheets using the static site generator of your choice. Feel free to look
   at [my templates](https://github.com/stusmall/stuartsmall.com/tree/eef7874ed7c6543d011d052d3e91bc09d02779ed/templates)
   for inspiration. I'm not a frontend developer, so you might find them lacking. "Lacking" being a polite term for
   "trainwreck".
2. Build a tool for converting Cooklang to
   Markdown.  [Feel free to steal mine](https://github.com/stusmall/stuartsmall.com/tree/eef7874ed7c6543d011d052d3e91bc09d02779ed/build-md)
   or use one of the parsers in the language of your choice to build your own. You might end up having similar unique
   needs and need your own custom logic.
3. Set up a GitHub Action workflow to build and deploy your site to
   Netlify.  [Once again, feel free to steal mine.](https://github.com/stusmall/stuartsmall.com/blob/eef7874ed7c6543d011d052d3e91bc09d02779ed/.github/workflows/netlify.yml)
4. Now create some recipes. You are done.  [Check out my live version to see if you like it and also be sure to try the butter chicken recipe](/recipes)
