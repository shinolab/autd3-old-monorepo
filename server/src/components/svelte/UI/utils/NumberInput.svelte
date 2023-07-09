<!--
File: NumberInput.svelte
Project: AUTD server
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
  export let value = 0;
  export let min: string | undefined = undefined;
  export let max: string | undefined = undefined;
  export let step = "1";

  let count_up = () => {
    value = value + 1;
    if (max === undefined) return;
    if (value > Number(max)) value = Number(max);
  };
  let count_down = () => {
    value = value - 1;
    if (min === undefined) return;
    if (value < Number(min)) value = Number(min);
  };
</script>

<label class="number-spinner">
  <input {id} type="number" bind:value {min} {max} {step} />
  <span
    class="spinner-up-wrap"
    role="button"
    tabindex={-1}
    on:click={count_up}
    on:keyup={count_up}
  >
    <span class="spinner-up" />
  </span>
  <span
    class="spinner-down-wrap"
    role="button"
    tabindex={-1}
    on:click={count_down}
    on:keydown={count_down}
  >
    <span class="spinner-down" />
  </span>
</label>

<style>
  input[type="number"]::-webkit-outer-spin-button,
  input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  input {
    padding: 0px 4px;
    border-radius: 3px;
    border: 1px solid var(--color-border-interactive-muted, #2b659b);
    background: var(--color-background-base-default, #101923);
    color: var(--color-text-base-default, #ffffff);
    width: 100%;
  }
  .number-spinner {
    position: relative;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    margin: auto;
    width: 100%;
    height: 26px;
  }
  .spinner-up-wrap {
    position: absolute;
    width: 20px;
    height: 13px;
    right: 0px;
    top: 0;
    margin: 0;
    box-sizing: border-box;
    display: block;
  }
  .spinner-up {
    border-bottom: 6px solid #4dacff;
    border-left: 4px solid transparent;
    border-right: 4px solid transparent;
    content: "";
    position: absolute;
    transform: translate(0%, -50%);
    top: 50%;
    right: 6px;
  }
  .spinner-down-wrap {
    position: absolute;
    width: 20px;
    height: 13px;
    right: 0px;
    top: 13px;
    margin: 0;
    box-sizing: border-box;
    display: block;
  }
  .spinner-down {
    border-top: 6px solid #4dacff;
    border-left: 4px solid transparent;
    border-right: 4px solid transparent;
    content: "";
    position: absolute;
    transform: translate(0px, -50%);
    top: 50%;
    right: 6px;
  }
</style>
