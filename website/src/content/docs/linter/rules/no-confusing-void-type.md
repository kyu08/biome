---
title: noConfusingVoidType (since v1.0.0)
---


Disallow `void` type outside of generic or return types.

`void` in TypeScript refers to a function return that is meant to be ignored. Attempting to use a void type outside of a return type or generic type argument is often a sign of programmer error. void can also be misleading for other developers even if used correctly.

>The `void` type means cannot be mixed with any other types, other than `never`, which accepts all types.
If you think you need this then you probably want the undefined type instead.


## Examples

### Invalid

```ts
type PossibleValues = number | void;
type MorePossibleValues = string | ((number & any) | (string | void));
```

<pre class="language-text"><code class="language-text">nursery/noConfusingVoidType.js:1:32 <a href="https://biomejs.dev/linter/rules/no-confusing-void-type">lint/nursery/noConfusingVoidType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">void is not valid as a constituent in a union type</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>type PossibleValues = number | void;
   <strong>   │ </strong>                               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>type MorePossibleValues = string | ((number &amp; any) | (string | void));
    <strong>3 │ </strong>
  
nursery/noConfusingVoidType.js:2:64 <a href="https://biomejs.dev/linter/rules/no-confusing-void-type">lint/nursery/noConfusingVoidType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">void is not valid as a constituent in a union type</span>
  
    <strong>1 │ </strong>type PossibleValues = number | void;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>type MorePossibleValues = string | ((number &amp; any) | (string | void));
   <strong>   │ </strong>                                                               <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
</code></pre>

```ts
function logSomething(thing: void) {}
```

<pre class="language-text"><code class="language-text">nursery/noConfusingVoidType.js:1:30 <a href="https://biomejs.dev/linter/rules/no-confusing-void-type">lint/nursery/noConfusingVoidType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">void is only valid as a return type or a type argument in generic type</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function logSomething(thing: void) {}
   <strong>   │ </strong>                             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```ts
interface Interface {
    prop: void;
}
```

<pre class="language-text"><code class="language-text">nursery/noConfusingVoidType.js:2:11 <a href="https://biomejs.dev/linter/rules/no-confusing-void-type">lint/nursery/noConfusingVoidType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">void is only valid as a return type or a type argument in generic type</span>
  
    <strong>1 │ </strong>interface Interface {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    prop: void;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
</code></pre>

```ts
let foo: void;
let bar = 1 as unknown as void;
let baz = 1 as unknown as void | string;
```

<pre class="language-text"><code class="language-text">nursery/noConfusingVoidType.js:1:10 <a href="https://biomejs.dev/linter/rules/no-confusing-void-type">lint/nursery/noConfusingVoidType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">void is only valid as a return type or a type argument in generic type</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>let foo: void;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>let bar = 1 as unknown as void;
    <strong>3 │ </strong>let baz = 1 as unknown as void | string;
  
nursery/noConfusingVoidType.js:2:27 <a href="https://biomejs.dev/linter/rules/no-confusing-void-type">lint/nursery/noConfusingVoidType</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">void is only valid as a return type or a type argument in generic type</span>
  
    <strong>1 │ </strong>let foo: void;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>let bar = 1 as unknown as void;
   <strong>   │ </strong>                          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>let baz = 1 as unknown as void | string;
    <strong>4 │ </strong>
  
</code></pre>

### Valid

```ts
function foo(): void {};
function doSomething(this: void) {}
function printArg<T = void>(arg: T) {}
logAndReturn<void>(undefined);
let voidPromise: Promise<void> = new Promise<void>(() => { });
let voidMap: Map<string, void> = new Map<string, void>();
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
