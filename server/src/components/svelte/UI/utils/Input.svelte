<!--
File: IpInput.svelte
Project: AUTD Server
Created Date: 06/07/2023
Author: Shun Suzuki
-----
Last Modified: 10/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

-->

<script lang="ts">
  export let id = "";
  export let value = "";
</script>

<input {id} type="text" bind:value />

<style>
  input {
    border-radius: 3px;
    border: 1px solid var(--color-border-interactive-muted, #2b659b);
    background: var(--color-background-base-default, #101923);
    color: var(--color-text-base-default, #ffffff);
  }
</style>
