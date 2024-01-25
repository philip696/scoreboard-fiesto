<template>
    <div>
        <div class="flex flex-col items-center justify-center w-full mb-8 ">
            <span class="text-4xl font-bold">{{ info.name }}</span>
        </div>
        <div class="flex flex-col items-center justify-center w-full mb-8">
            <span class="text-4xl font-bold mb-2">Score</span>
            <div class="flex flex-row-reverse items-center justify-around w-full">
                <button @click="emitEvent('score_step_event', { step: 'up', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded">
                    +
                </button>
                <span class="text-4xl font-bold">{{ info.score }}</span>
                <button @click="emitEvent('score_step_event', { step: 'down', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded">
                    -
                </button>
            </div>
        </div>
        <div class="flex flex-col items-center justify-center w-full mb-8">
            <span class="text-4xl font-bold mb-2">Foul</span>
            <div class="flex flex-row-reverse items-center justify-around w-full">
                <button @click="emitEvent('foul_step_event', { step: 'up', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded">
                    +
                </button>
                <span class="text-4xl font-bold">{{ info.foul }}</span>
                <button @click="emitEvent('foul_step_event', { step: 'down', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded">
                    -
                </button>
            </div>
        </div>
        <div class="flex flex-col items-center justify-center w-full mb-8">
            <span class="text-4xl font-bold mb-2">Timeout</span>
            <div class="flex flex-row-reverse items-center justify-around w-full">
                <button @click="emitEvent('timeout_step_event', { step: 'up', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded">
                    +
                </button>
                <span class="text-4xl font-bold">{{ info.timeout }}</span>
                <button @click="emitEvent('timeout_step_event', { step: 'down', team: teamname })"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded">
                    -
                </button>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { emit } from "@tauri-apps/api/event";
import type { TeamInfo } from "~/types/TeamInfo";

export default {
    props: {
        info: {
            type: Object as () => TeamInfo,
            required: true
        },
        teamname: {
            type: String,
            required: true
        }
    },
    methods: {
        emitEvent(event: string, data: any) {
            emit(event, data);
        },
    },
}
</script>

<style></style>