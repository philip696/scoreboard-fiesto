<template>
    <div class="fixed w-screen h-screen bg-slate-400">
        <div class="flex flex-col items-center justify-center h-screen">
            <div class="flex items-center justify-between">
                <TeamController teamname="teamA" :info="teamA"
                    class="flex flex-col items-center justify-center h-full w-1/3" />
                <div class="flex flex-col items-center justify-center h-full w-1/3">
                    <span class="text-3xl font-bold mb-3">CONTROLLER</span>
                    <div class="relative inline-block py-2 mb-2">
                        <select @change="updateQuarter"
                            class="block appearance-none w-full bg-white border border-gray-400 hover:border-gray-500 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
                            v-model="quarter" placeholder="Quarter">
                            <option disabled value="0">Quarter</option>
                            <option value="1">
                                Quarter 1
                            </option>
                            <option value="2">
                                Quarter 2
                            </option>
                            <option value="3">
                                Quarter 3
                            </option>
                            <option value="4">
                                Quarter 4
                            </option>
                            <option value="5">
                                Overtime
                            </option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                                <path d="M5.5 7L10 11.5 14.5 7z" />
                            </svg>
                        </div>
                    </div>
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="startTimer">
                        Start Timer
                    </button>
                    <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="stopTimer">
                        Stop Timer
                    </button>
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="startTimerTimeout">
                        Start Timeout
                    </button>
                    <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="stopTimerTimeout">
                        Stop Timeout
                    </button>

                    <div class="flex items-center justify-center">
                        <h1 class="text-3xl font-bold underline">
                            {{ formatedTime }}
                        </h1>
                    </div>
                </div>
                <TeamController teamname="teamB" :info="teamB"
                    class="flex flex-col items-center justify-center h-full w-1/3" />
            </div>
            <div class="flex items-center justify-around">
                <button @click="showBanner('scorer_url')"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 m-2 rounded">Show Scorer</button>
                <button @click="showBanner('dark_statistic_url')"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 m-2 rounded">Show Statistic
                    (Dark)</button>
                <button @click="showBanner('light_statistic_url')"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 m-2 rounded">Show Statistic
                    (Light)</button>
                <button @click="showBanner('man_of_the_match_url')"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 m-2 rounded">Show MOTM</button>
                <button @click="showBanner('top_player_url')"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 m-2 rounded">Show Top
                    Player</button>
            </div>
            <div @click="openConfig" class="flex items-center ml-10 w-full">
                <Icon name="mynaui:config" class="text-3xl hover:cursor-pointer hover:bg-blue-400" />
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { emit, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { WebviewWindow } from '@tauri-apps/api/window';
import type { TeamInfo } from '~/types/TeamInfo';

type PreviewUrl = {
    'scorer_url': string,
    'dark_statistic_url': string,
    'light_statistic_url': string,
    'man_of_the_match_url': string,
    'top_player_url': string
}

export default {
    data() {
        return {
            time: 0,
            teamA: {
                name: 'Tim A',
                picture: '',
                score: 0,
                foul: 0,
                timeout: 0
            } as TeamInfo,
            teamB: {
                name: 'Tim B',
                picture: '',
                score: 0,
                foul: 0,
                timeout: 0
            } as TeamInfo,
            quarter: 0,
            previewUrl: '',
            isBannerShown: false,
            listUrl: {
            } as PreviewUrl
        }
    },
    mounted() {
        listen('timer_event', (event: any) => {
            this.time = event.payload.value
        });

        listen('team_a_event', (event: any) => {
            this.teamA = event.payload.teamA
        });

        listen('team_b_event', (event: any) => {
            this.teamB = event.payload.teamB
        });

        invoke('get_config').then((state: any) => {
            this.listUrl = state;
        })
    },
    computed: {
        formatedTime() {
            const milliseconds = (this.time % 1000) / 10;
            const seconds = Math.floor(this.time / 1000) % 60;
            const minutes = Math.floor(this.time / (1000 * 60)) % 60;

            const strMinutes = String((minutes < 10) ? "0" + minutes.toFixed(0) : minutes.toFixed(0));
            const strSeconds = String((seconds < 10) ? "0" + seconds.toFixed(0) : seconds.toFixed(0));
            const strMilliseconds = String((milliseconds < 10) ? "0" + milliseconds.toFixed(0) : milliseconds.toFixed(0));

            return strMinutes + ":" + strSeconds + "." + strMilliseconds;
        }
    },
    methods: {
        async startTimer() {
            console.log(this.time)
            if (this.time > 0) {
                emit('start_timer_event', { initialTime: this.time })
            } else {
                emit('start_timer_event', { initialTime: 600000 })
            }
        },
        async openConfig() {
            let config = new WebviewWindow('configurationpage', {
                alwaysOnTop: true
            })
            config.show();
        },
        async stopTimer() {
            emit('stop_timer_event')
        },
        async startTimerTimeout() {
            emit('start_timeout_event', { initialTime: 60000 })
        },
        async stopTimerTimeout() {
            emit('stop_timeout_event')
        },
        async showBanner(key: "scorer_url" | "dark_statistic_url" | "light_statistic_url" | "man_of_the_match_url" | "top_player_url") {
            console.log(this.listUrl[key]);
            emit('show_banner', { url: this.listUrl[key] });
        },
        async updateQuarter() {
            emit('quarter_event', { quarter: this.quarter })
        }
    }
}
</script>

<style></style>