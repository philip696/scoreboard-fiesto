<template>
    <div class="fixed container-body font-basementGrotesque bg-[url('~/assets/img/mock.jpg')]">
        <div class="wrapper-score">
            <img src="../assets//img/light-slim.png" alt="" class="light-left" />
            <img src="../assets//img/light-slim.png" alt="" class="light-right" />
            <div class="header-score">
                <div class="bg-header"></div>
                <div class="logo-header">
                    <img src="../assets/img/logo.svg" />
                </div>
                <div class="label-header">
                    <div class="left-label label-score">Terang</div>
                    <div class="right-label label-score">Gelap</div>
                </div>
            </div>
            <div class="body-score">
                <img src="../assets//img/light-big.png" alt="" class="light-big" />
                <div class="score-block">
                    <div class="left-score score-count">
                        <div>{{ teamA.score }}</div>
                        <img src="../assets//img/light-slim.png" alt="" class="light-score" />
                    </div>
                    <div class="right-score score-count">
                        <div>{{ teamB.score }}</div>
                        <img src="../assets//img/light-slim.png" alt="" class="light-score" />
                    </div>
                </div>
            </div>
            <div class="footer-score">
                <div class="foul-team left-foul">
                    <div>
                        <h3>TEAM FOUL</h3>
                        <div class="foul-count">{{ teamA.foul }}</div>
                    </div>
                    <div class="timout-count">
                        <div v-for="n in 3" :class="n <= teamA.timeout ? 'active' : ''" class="list-timout"></div>
                    </div>
                </div>
                <div class="timer">{{ formatedTime }}</div>
                <div class="foul-team right-foul">
                    <div class="timout-count">
                        <div v-for="n in 3" :class="n <= teamB.timeout ? 'active' : ''" class="list-timout"></div>
                    </div>
                    <div>
                        <h3>TEAM FOUL</h3>
                        <div class="foul-count">{{ teamB.foul }}</div>
                    </div>
                </div>
            </div>
        </div>
        <!-- <div class="flex items-center justify-center w-screen h-screen bg-gray-700">
            <div class="flex items-center justify-center h-screen w-2/5">
                <TeamInfo name="Team 1" :info="teamA" />
            </div>
            <div class="flex flex-col items-center justify-center w-1/5">
                <div class="flex items-center justify-center bg-slate-300 rounded-lg px-8 py-4">
                    <span v-show="quarterName !== ''" class="text-4xl font-bold">
                        {{ quarterName }}
                    </span>
                </div>
                <span class="text-8xl font-bold">
                    {{ formatedTime }}
                </span>
                <span v-if="isTimeout" class="text-6xl font-bold text-red-400">
                    {{ formatedTimeout }}
                </span>
            </div>
            <div class="flex items-center justify-center h-screen w-2/5">
                <TeamInfo name="Team 2" :info="teamB" />
            </div>
        </div> -->
        <div v-show="isBannerShown" class="fixed inset-0 w-screen h-screen flex items-center justify-center bg-opacity-0">
            <div class="relativeshadow-lg w-full max-w-2xl mx-4 my-8">
                <span class="absolute top-0 right-0 cursor-pointer p-4" @click="isBannerShown = false">x</span>
                <div class="p-4">
                    <iframe class="w-full" :src="previewUrl" frameborder="0"></iframe>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import type { TeamInfo } from '~/types/TeamInfo';
import { doc, updateDoc } from 'firebase/firestore';

export default {
    data() {
        return {
            time: 0,
            timeout: 0,
            timer: null as any,
            timerTimeout: null as any,
            updateCounter: 0,
            isRunning: false,
            isTimeout: false,
            isBannerShown: false,
            previewUrl: '',
            quarter: 0,
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
        }
    },
    async mounted() {
        await listen('start_timer_event', (event: any) => {
            this.startTimer(event.payload.initialTime);
        })
        await listen('stop_timer_event', (event: any) => {
            this.stopTimer();
        })
        await listen('start_timeout_event', (event: any) => {
            this.startTimeout(event.payload.team, event.payload.initialTime);
        })
        await listen('stop_timeout_event', (event: any) => {
            this.stopTimeout();
        })
        await listen('show_banner', (event: any) => {
            this.showBanner(event.payload.url);
        })
        await listen('quarter_event', (event: any) => {
            console.log("quarter : ", event.payload.quarter)
            this.quarter = event.payload.quarter;
        })

        await listen('score_step_event', (event: any) => {
            switch (event.payload.step) {
                case 'up':
                    switch (event.payload.team) {
                        case 'teamA':
                            this.teamA.score += 1;
                            break;

                        case 'teamB':
                            this.teamB.score += 1;
                            break;

                        default:
                            break;
                    }
                    break;

                case 'down':
                    switch (event.payload.team) {
                        case 'teamA':
                            this.teamA.score = this.teamA.score - 1 < 0 ? 0 : this.teamA.score - 1;
                            break;

                        case 'teamB':
                            this.teamB.score = this.teamB.score - 1 < 0 ? 0 : this.teamB.score - 1;
                            break;

                        default:
                            break;
                    }
                    break;

                default:
                    break;
            }
        })
        await listen('foul_step_event', (event: any) => {
            switch (event.payload.step) {
                case 'up':
                    switch (event.payload.team) {
                        case 'teamA':
                            this.teamA.foul += 1;
                            break;

                        case 'teamB':
                            this.teamB.foul += 1;
                            break;

                        default:
                            break;
                    }
                    break;

                case 'down':
                    switch (event.payload.team) {
                        case 'teamA':
                            this.teamA.foul = this.teamA.foul - 1 < 0 ? 0 : this.teamA.foul - 1;
                            break;

                        case 'teamB':
                            this.teamB.foul = this.teamB.foul - 1 < 0 ? 0 : this.teamB.foul - 1;
                            break;

                        default:
                            break;
                    }
                    break;

                default:
                    break;
            }
        })
        await listen('timeout_step_event', (event: any) => {
            switch (event.payload.step) {
                case 'up':
                    switch (event.payload.team) {
                        case 'teamA':
                            this.teamA.timeout = this.teamA.timeout + 1 > 5 ? 5 : this.teamA.timeout + 1;
                            break;

                        case 'teamB':
                            this.teamB.timeout = this.teamB.timeout + 1 > 5 ? 5 : this.teamB.timeout + 1;
                            break;

                        default:
                            break;
                    }
                    break;

                case 'down':
                    switch (event.payload.team) {
                        case 'teamA':
                            this.teamA.timeout = this.teamA.timeout - 1 < 0 ? 0 : this.teamA.timeout - 1;
                            break;

                        case 'teamB':
                            this.teamB.timeout = this.teamB.timeout - 1 < 0 ? 0 : this.teamB.timeout - 1;
                            break;

                        default:
                            break;
                    }
                    break;

                default:
                    break;
            }
        })
    },
    watch: {
        teamA: {
            handler(newVal, oldVal) {
                console.log(newVal, oldVal)
                emit('team_a_event', {
                    teamA: {
                        name: this.teamA.name,
                        score: this.teamA.score,
                        foul: this.teamA.foul,
                        timeout: this.teamA.timeout
                    },
                })
            },
            deep: true
        },
        teamB: {
            handler(newVal, oldVal) {
                console.log(newVal, oldVal)
                emit('team_b_event', {
                    teamB: {
                        name: this.teamB.name,
                        score: this.teamB.score,
                        foul: this.teamB.foul,
                        timeout: this.teamB.timeout
                    },
                })
            },
            deep: true
        }
    },
    computed: {
        formatedTime() {
            if (this.time > 60000) {
                const milliseconds = (this.time % 1000) / 10;
                const seconds = Math.floor(this.time / 1000) % 60;
                const minutes = Math.floor(this.time / (1000 * 60)) % 60;

                const strMinutes = String((minutes < 10) ? "0" + minutes.toFixed(0) : minutes.toFixed(0));
                const strSeconds = String((seconds < 10) ? "0" + seconds.toFixed(0) : seconds.toFixed(0));
                const strMilliseconds = String((milliseconds < 10) ? "0" + milliseconds.toFixed(0) : milliseconds.toFixed(0));

                return strMinutes + ":" + strSeconds;// + "." + strMilliseconds;
            } else {
                const milliseconds = (this.time % 1000) / 10;
                const seconds = Math.floor(this.time / 1000) % 60;
                const minutes = Math.floor(this.time / (1000 * 60)) % 60;
                return `${seconds}.${milliseconds}`; //minutes + ":" + seconds;
            }
        },
        formatedTimeout() {
            const milliseconds = (this.timeout % 1000) / 10;
            const seconds = Math.floor(this.timeout / 1000) % 60;
            const minutes = Math.floor(this.timeout / (1000 * 60)) % 60;

            const strMinutes = String((minutes < 10) ? "0" + minutes.toFixed(0) : minutes.toFixed(0));
            const strSeconds = String((seconds < 10) ? "0" + seconds.toFixed(0) : seconds.toFixed(0));
            const strMilliseconds = String((milliseconds < 10) ? "0" + milliseconds.toFixed(0) : milliseconds.toFixed(0));

            return strMinutes + ":" + strSeconds;
        },
        quarterName() {
            switch (String(this.quarter)) {
                case "1":
                    return 'Q1';
                case "2":
                    return 'Q2';
                case "3":
                    return 'Q3';
                case "4":
                    return 'Q4';
                case "5":
                    return 'OT';
                default:
                    return '';
            }
        }
    },
    methods: {
        startTimer(initialTime: number = 600000) {
            // const adsWindow = new WebviewWindow('ads');
            // adsWindow.show();
            console.log(initialTime);
            if (!this.isRunning) {
                this.isRunning = true;
                this.time = initialTime;

                this.timer = setInterval(() => {
                    this.updateCounter += 1;
                    if (this.updateCounter >= 10) {
                        this.updateCounter = 0;
                        invoke('update_time', { time: this.formatedTime });
                        this.sendToFirebase();
                        emit('timer_event', { value: this.time });
                    }
                    this.time -= 10; // Increment every 10 milliseconds
                    if (this.time <= 0) {
                        this.time = 0;
                        this.stopTimer();
                    }
                }, 10);
            }
        },
        stopTimer() {
            this.isRunning = false;
            clearInterval(this.timer);
        },
        startTimeout(team: String, initialTime: number = 60000) {
            console.log(team)
            if (!this.isTimeout) {
                this.isTimeout = true;
                this.timeout = initialTime;
                this.timerTimeout = setInterval(() => {
                    console.log(this.timeout)
                    if (this.timeout <= 0) {
                        this.stopTimeout();
                    } else {
                        this.timeout -= 10;
                    }
                }, 10);
            }
        },
        stopTimeout() {
            this.isTimeout = false;
            clearInterval(this.timerTimeout);
        },
        showBanner(url: string) {
            this.previewUrl = url;
            this.isBannerShown = true;
            setTimeout(() => {
                this.isBannerShown = false;
            }, 3000);
        },
        async sendToFirebase() {
            const { $firestore: firestore } = useNuxtApp();

            // Reference to a document in Firestore
            const docRef = doc(firestore, 'scoreboard_timer', 'stream1');

            try {
                await updateDoc(docRef, {
                    minutes: this.formatedTime, // Fields to update
                    // ... more fields to update
                });
                console.log('Document updated successfully');
            } catch (error) {
                console.error('Error updating document: ', error);
            }

        }
    },
}
</script>

<style scoped>
/* :root {
    --light1: #67bbf7;
    --light2: #1c448c;
    --light3: #183a77;
    --lightcolor: #fff;
    --dark1: #d92933;
    --dark2: #8a1b2b;
    --dark3: #771725;
    --darkcolor: #fff;
}

@font-face {
    font-family: BasementGrotesque;
    src: url("BasementGrotesque.otf") format("opentype");
} */

.container-body {
    margin: 0;
    background: url(../assets/img/background.jpg);
    background-size: 100% auto;
    font-family: BasementGrotesque;
    height: 100vh;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.logo-header img {
    max-width: 100%;
    display: block;
}

div.wrapper-score:before {
    content: "";
    display: block;
    position: absolute;
    width: calc(100% - 24px);
    height: calc(100% - 24px);
    background: #202538;
    top: 12px;
    left: 11px;
    border-radius: 20px;
}

div.wrapper-score {
    width: 1005px;
    margin: 0 auto;
    background: cadetblue;
    background: linear-gradient(143deg, #ffffff, #acabaa);
    border-image-slice: 1;
    border-radius: 30px;
    position: relative;
    margin-top: 60px;
}

@media(min-width: 1700px) {
    div.wrapper-score {
        transform: scale(1.3);
    }
}

.logo-header img {
    width: 100%;
    height: 100%;
    object-fit: fill;
}

.logo-header {
    width: 125px;
    height: 125px;
    margin: 0 auto;
    background: #202539;
    border-radius: 100%;
    padding: 10px;
    top: -64px;
    position: relative;
    z-index: 1;
    border: 5px solid #fff;
}

.header-score:after {
    content: "";
    display: block;
    position: absolute;
    width: 60%;
    height: 100%;
    right: -40px;
    top: 0;
    background: linear-gradient(180deg,
            rgba(254, 255, 254, 1) 25%,
            rgba(164, 164, 164, 1) 52%,
            rgb(147 147 147) 62%,
            rgb(199 199 199) 91%);
    transform: skew(-8deg);
    border-radius: 5px 5px 15px 5px;
}

.header-score:before {
    content: "";
    display: block;
    position: absolute;
    width: 60%;
    height: 100%;
    left: -40px;
    top: 0;
    background: linear-gradient(180deg,
            rgba(254, 255, 254, 1) 25%,
            rgba(164, 164, 164, 1) 52%,
            rgb(147 147 147) 62%,
            rgb(199 199 199) 91%);
    transform: skew(8deg);
    border-radius: 5px 5px 5px 15px;
}

.header-score {
    position: relative;
    height: 110px;
    background: #fff;
}

.label-score {
    width: 31%;
    background: linear-gradient(180deg,
            var(--light1) 20%,
            var(--light2) 96%);
    color: var(--lightcolor);
    font-weight: normal;
    text-transform: uppercase;
    letter-spacing: 2px;
    text-align: center;
    border-radius: 10px 10px 0 0;
    padding: 27px 10px 7px;
    font-size: 26pt;
    position: relative;
}

.label-score:after {
    content: "";
    display: block;
    position: absolute;
    width: calc(100% + 12px);
    background: var(--light2);
    height: 20px;
    left: 0;
    top: 100%;
    margin-top: -1px;
    border-radius: 0 0 0px 10px;
}

.label-score:before {
    content: "";
    display: block;
    position: absolute;
    left: 100%;
    background: var(--light3);
    height: 25px;
    width: 28px;
    top: 100%;
    border-radius: 0 0 100px 100px;
    z-index: 1;
    margin-top: -6px;
    margin-bottom: 5px;
}

.right-label.label-score {
    background: linear-gradient(180deg, var(--dark1) 20%, var(--dark2) 96%);
    color: var(--darkcolor);
}

.right-label.label-score:before {
    left: auto;
    background: var(--dark3);
    right: 100%;
}

.right-label.label-score:after {
    background: var(--dark2);
    border-radius: 0 0 10px 0px;
    left: auto;
    right: 0;
}

.label-header {
    display: flex;
    justify-content: space-around;
    align-items: center;
    position: absolute;
    width: 100%;
    bottom: -6px;
    z-index: 2;
}

.body-score {
    padding-top: 50px;
}

.score-block {
    position: relative;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-around;
}

.score-count div {
    position: relative;
    transform: skew(-8deg);
}

.score-count {
    background: linear-gradient(143deg, #ffffff, #dededc);
    width: 40%;
    position: relative;
    color: var(--lightcolor);
    font-size: 110pt;
    letter-spacing: 2px;
    text-align: center;
    padding: 40px 0 10px;
    transform: skew(8deg);
    border-radius: 18px;
    margin-left: 40px;
}

.score-count:before {
    content: "";
    display: block;
    position: absolute;
    width: calc(100% - 20px);
    top: 10px;
    border-radius: 12px;
    height: calc(100% - 20px);
    left: 10px;
    background: linear-gradient(180deg,
            var(--light1) 20%,
            var(--light2) 60%);
}

.right-score.score-count {
    transform: skew(-8deg);
    margin-right: 40px;
    margin-left: 0px;
    color: var(--darkcolor);
}

.right-score.score-count div {
    transform: skew(8deg);
}

.right-score.score-count:before {
    background: linear-gradient(180deg, var(--dark1) 20%, var(--dark2) 60%);
}

.footer-score {
    position: relative;
    width: 100%;
    color: #fff;
    display: flex;
    justify-content: space-around;
    align-items: end;
    margin-bottom: 40px;
    margin-top: 30px;
}

.timer {
    color: #ffbc58;
    font-size: 56pt;
    border: 5px solid #e1e1e1;
    border-radius: 15px;
    padding: 10px 20px 1px;
    width: 34%;
    text-align: center;
}

.foul-count {
    background: linear-gradient(180deg,
            var(--light1) 20%,
            var(--light2) 60%);
    text-align: center;
    font-size: 50pt;
    padding: 10px 0 0px;
    border: 5px solid #fff;
    color: var(--lightcolor);
    line-height: 1;
    border-radius: 15px;
}

.timout-count {
    display: flex;
    flex-direction: column-reverse;
}

.right-foul .foul-count {
    background: linear-gradient(180deg, var(--dark1) 20%, var(--dark2) 60%);
    color: var(--darkcolor);
}

.left-foul {
    margin-left: 60px;
}

.right-foul {
    margin-right: 60px;
}

.foul-team h3 {
    margin: 0 0 8px;
    text-align: center;
    letter-spacing: 1px;
}

.right-foul .list-timout {
    margin-right: 40px;
    margin-left: 0px;
}

.right-foul .list-timout.active {
    background: var(--dark1);
}

.list-timout.active {
    background: var(--light1);
}

.list-timout {
    width: 20px;
    height: 20px;
    border: 3px solid #999;
    border-radius: 100%;
    margin-top: 10px;
    margin-left: 40px;
}

.foul-team {
    display: flex;
    align-items: end;
}

img.light-score {
    position: absolute;
    z-index: 999;
    top: 68px;
    transform: skew(-8deg) rotate(90deg) translateX(-50%);
    width: 70px;
    left: 41%;
}

img.light-big {
    position: absolute;
    max-width: 100%;
    margin: 0 auto;
}

img.light-left {
    position: absolute;
    left: -45px;
    z-index: -1;
    height: 100%;
}

img.light-right {
    position: absolute;
    right: -45px;
    z-index: -1;
    height: 100%;
}
</style>