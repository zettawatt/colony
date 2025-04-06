import type { SvelteComponent } from "svelte";
import GitHub from "./github.svelte";
import Logo from "./logo.svelte";

export type Icon = SvelteComponent;

export const Icons = {
	logo: Logo,
	gitHub: GitHub,
};
