# Challenge: Shift Left challenge

You get a [JSON object](dataset.json) that contains an [HTML document](gridpage.html) that shows a lot of differently coloured arrows:

![arrow grid with coloured arrows in all directions](arrows.png)

The HTML is something like this: 

```html
<div class="turngrid">
  <div class="patch" style="color: rgb(255, 238, 0); transform: rotate(200deg);">⬅</div>
  <div class="patch" style="color: rgb(255, 255, 0); transform: rotate(195deg);">⬅</div>
  <div class="patch" style="color: rgb(238, 255, 0); transform: rotate(93deg);">⬅</div>
  <div class="patch" style="color: rgb(221, 255, 0); transform: rotate(66deg);">⬅</div>
<!-- lots more -->
</div>
```

The task is to find all the arrows that point to the right.

Give back the amount of arrows pointing to the right as an integer, f.e. `45`. 