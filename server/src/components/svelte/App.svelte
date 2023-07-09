<!--
File: App.svelte
Project: AUTD server
Created Date: 07/07/2023
Author: Shun Suzuki
-----
Last Modified: 10/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

-->

<script lang="ts">
  import type { Options } from "./UI/options";

  import { invoke } from "@tauri-apps/api";
  import { appWindow } from "@tauri-apps/api/window";
  import { TauriEvent } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  import LeftPanel from "./LeftPanel.svelte";
  import RightPanel from "./RightPanel.svelte";

  let options: null | Options = null;

  onMount(async () => {
    options = await invoke("load_settings", {});
  });

  const handleUnload = async () => {
    if (options) {
      let args = {
        options: JSON.stringify(options),
      };
      await invoke("save_settings", args);
    }
  };

  appWindow.listen(TauriEvent.WINDOW_CLOSE_REQUESTED, async () => {
    await handleUnload();
    await appWindow.close();
  });
</script>

<div>
  {#if options}
    <LeftPanel {options} />
  {/if}
  <RightPanel />
</div>

<style>
  div {
    display: flex;
    width: 100%;
    align-items: flex-start;
    gap: 10px;
    flex-shrink: 0;

    padding: 10px;

    height: 100vh;
    box-sizing: border-box;
  }
</style>
