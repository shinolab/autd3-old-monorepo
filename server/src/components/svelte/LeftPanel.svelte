<script>
    import { Tabs, Tab, TabList, TabPanel } from "svelte-tabs";
    import { platform } from "@tauri-apps/api/os";

    import TwinCAT from "./UI/TwinCAT.svelte";
    import SOEM from "./UI/SOEM.svelte";
    import Simulator from "./UI/Simulator.svelte";

    import { onMount } from "svelte";

    let platformName = "";

    onMount(async () => {
        platformName = await platform();
    });
</script>

<div>
    <Tabs>
        <TabList>
            {#if platformName == "win32"}
                <Tab>TwinCAT</Tab>
            {/if}
            <Tab>SOEM</Tab>
            <Tab>Simulator</Tab>
        </TabList>

        {#if platformName == "win32"}
            <TabPanel>
                <TwinCAT />
            </TabPanel>
        {/if}

        <TabPanel>
            <SOEM />
        </TabPanel>

        <TabPanel>
            <Simulator />
        </TabPanel>
    </Tabs>
</div>

<style>
    div {
        display: flex;
        width: 360px;
        flex-direction: column;
        align-items: flex-start;
        flex-shrink: 0;
        align-self: stretch;
    }

    :global(.svelte-tabs) {
        width: 100%;
    }

    :global(.svelte-tabs ul.svelte-tabs__tab-list) {
        border-bottom: none;
    }

    :global(.svelte-tabs li.svelte-tabs__tab) {
        box-sizing: border-box;
        color: var(--color-text-base-default, #ffffff);
    }

    :global(.svelte-tabs li.svelte-tabs__selected) {
        color: #4dacff;
    }

    :global(.svelte-tabs div.svelte-tabs__tab-panel) {
        color: var(--color-text-base-default, #ffffff);
    }
</style>
