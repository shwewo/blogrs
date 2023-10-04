cloudflare-workers 2023-05-16 About cloudflare workers

![Workers](https://blog.cloudflare.com/content/images/2019/06/45DEDC7B-B31F-461C-B786-12FBAF1A5391.png)
# What's about it?

Recently i discovered awesome thing — Cloudflare Workers. Pros — it's free, it can provide a free domain name, it has 1GB of storage for free, and it's freaking fast! And it can be written entirely in Rust.
[Native Rust support in workers](https://blog.cloudflare.com/workers-rust-sdk/)

# How to start?

At first, register an account in cloudflare.
Then, select at dashboard, you should see "Workers".
![Workers](https://i.imgur.com/lEYl4f2.png)

Then, you can easily create new service. Click "Create a service" button.

![Workers](https://i.imgur.com/VfCehbH.png)

Choose a template, e.g. "HTTP handler"
![Workers](https://i.imgur.com/lXhovy0.png)

No custom domain needed, cloudflare will give you an free domain on `domain.yourworkerdomain.workers.dev` (Though you can use your own domain for it). And  you can name it whatever you want.

And from now on, you can see your worker dashboard
![Workers](https://i.imgur.com/N6DXKbU.png)

In here, you can click "Quick edit" button, and there will be a small "IDE" for quick edit (Though it's highly discouraged to use that tool, it's better to use wrangler CLI).

![Workers](https://i.imgur.com/eI5k26n.png)

That's it! Your first worker is online and you can access it via given URL.