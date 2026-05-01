<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { marked } from "marked";

interface Message {
    content: string;
    role: "llm" | "user";
}

const messages = ref<Message[]>([]);
const txt = ref("");
const is_waiting = ref(false);

function to_html(text: string): string {
    if (!text) return "";
    return marked.parse(text) as string;
}

const bottomObserver = ref(null);
const blur = ref(false);
let observer: IntersectionObserver | null = null;

onMounted(() => {
    observer = new IntersectionObserver(
        (entries) => {
            const entry = entries[0];

            if (entry.isIntersecting) {
                blur.value = false;
            } else {
                blur.value = true;
            }
        },
        {
            root: document.getElementById("container"),
            threshold: 1.0,
        },
    );

    if (bottomObserver.value) {
        observer.observe(bottomObserver.value);
    }
});

onUnmounted(() => {
    if (observer) observer.disconnect();
});

async function send() {
    if (txt.value.trim() !== "" && !is_waiting.value) {
        const userMessage = txt.value;
        messages.value.push({
            role: "user",
            content: userMessage,
        });

        txt.value = "";
        is_waiting.value = true;

        try {
            const response = await fetch(
                `http://127.0.0.1:8000/?message=${encodeURIComponent(userMessage)}`,
            );
            const text = await response.text();

            messages.value.push({
                role: "llm",
                content: to_html(text),
            });
        } catch (error) {
            console.error("Błąd fetch:", error);
        } finally {
            is_waiting.value = false;
        }
    }
}
</script>

<template>
    <div id="blur" :style="{ opacity: blur ? '1' : '0' }" />

    <div
        id="container"
        :class="{
            started: messages.length > 0,
        }"
    >
        <div
            ref="bottomObserver"
            style="height: 1px; z-index: 99; position: relative; top: 60px"
        />

        <div id="message" v-if="messages.length > 0">
            <div
                id="mess"
                v-for="message in messages"
                v-html="message.content"
                :class="{
                    llm: message.role === 'llm',
                    user: message.role === 'user',
                }"
            />
        </div>

        <div id="animation" v-if="is_waiting">
            <span></span><span></span><span></span><span></span>
        </div>

        <form
            id="form"
            @submit.prevent="send"
            :style="{ opacity: is_waiting ? '0.5' : '1' }"
        >
            <input
                id="text"
                type="text"
                placeholder="Napisz cos..."
                v-model="txt"
            />
            <button id="submit" type="submit">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="76"
                    height="76"
                    fill="currentColor"
                    class="bi bi-arrow-right-short"
                    viewBox="0 0 16 16"
                >
                    <path
                        fill-rule="evenodd"
                        d="M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8"
                    />
                </svg>
            </button>
        </form>
    </div>
</template>

<style>
body {
    background-image: url("/back1.jpg");
    background-repeat: no-repeat;
    background-size: cover;
}

@keyframes start {
    to {
        height: 80dvh;
        background-color: rgba(0, 0, 0, 0.4);
        border: 2px solid rgba(255, 255, 255, 0.1);
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.8);
        backdrop-filter: blur(20px);
    }
}
#animation {
    margin: 10px auto;
    display: flex;
    padding-left: 40px;
    width: 844px;
}
#blur {
    width: 78dvw;
    transition: 0.3s ease-in-out;
    border-radius: 0 0 100% 100%;
    height: 70px;
    margin-bottom: -70px;
    z-index: 2;
    background: radial-gradient(
        50% 100% at center top,
        rgba(28, 0, 41, 0.95) 0%,
        rgba(28, 0, 41, 0.7) 20%,
        rgba(28, 0, 41, 0.3) 60%,
        rgba(28, 0, 41, 0) 90%
    );
    pointer-events: none;
}
#animation span {
    width: 12px;
    height: 12px;
    background: #f2deff;
    display: inline-block;
    border-radius: 50%;
    margin: 0 2px;
    animation: bounce 1.4s infinite ease-in-out both;
}
#animation span:nth-child(1) {
    animation-delay: -0.48s;
}
#animation span:nth-child(2) {
    animation-delay: -0.32s;
}
#animation span:nth-child(3) {
    animation-delay: -0.16s;
}
#animation span:nth-child(4) {
    animation-delay: -0.08s;
}

@keyframes bounce {
    0%,
    80%,
    100% {
        transform: scale(0);
    }
    40% {
        transform: scale(1);
    }
}

#container {
    position: relative;
    width: 83%;
    margin: 0 auto;
    border-radius: 80px;
    corner-shape: squircle;
    height: 108px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: center;
    backdrop-filter: blur(0px);
    background-color: rgba(0, 0, 0, 0);
    border: 2px solid rgba(255, 255, 255, 0);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0);
    justify-content: flex-start;
    overflow-y: auto;
    scrollbar-width: none;
    overflow-x: hidden;
    padding-bottom: 14px;

    &.started {
        animation: start 0.3s forwards;
    }
}

#form {
    display: flex;
    position: sticky;
    bottom: 20px;
    z-index: 10;
    margin-top: auto;
    justify-content: space-around;
    background: rgba(28, 0, 41, 0.6);
    backdrop-filter: blur(20px) brightness(1.2);
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-top: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 50px;
    corner-shape: squircle;
    padding: 20px;
    width: 800px;
    transition: 0.5s ease-in-out;
}

#form:focus-within {
    border: 1px solid rgba(255, 255, 255, 0.5);
}

#text {
    font-size: 30px;
    width: 80%;
    color: #f2deff;
    background: none;
    border: none;
}

#text::placeholder {
    color: #b3a4bd;
}

#text:focus {
    outline: none;
}

#submit {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 50px;
    height: 50px;
    background-color: #c200ff;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-right: 1px solid rgba(255, 255, 255, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    border-radius: 50%;
    corner-shape: squircle;
    cursor: pointer;
    margin-left: 20px;
    rotate: 270deg;
    transition: 0.2s ease-in-out;
}

#submit:hover {
    background-color: #f2deff;
}

#submit:hover svg {
    color: #56207a;
}

#submit svg {
    transition: 0.2s ease-in-out;
    color: #f2deff;
}

#message {
    width: 842px;
    margin: 30px auto 20px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    border-radius: 50px;
    overflow-wrap: anywhere;
}

#mess {
    padding: 15px 15px 15px 20px;
    max-width: 550px;
    background: #d22ffe;
    text-align: left;
    margin: 10px 0;
    color: #f2deff;

    font-size: 30px;
    line-height: 130%;
    border-radius: 25px 5px 25px 25px;
    min-width: 30px;

    &.llm {
        margin-right: auto;
        margin-top: 20px;
        border-radius: 25px 25px 25px 5px;
        background: rgba(28, 0, 41, 0.6);
    }
}
#mess:first-of-type {
    margin-top: 30px;
}
#mess:last-of-type {
    margin-bottom: 30px;
}
</style>
