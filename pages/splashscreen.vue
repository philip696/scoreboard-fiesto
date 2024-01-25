<template>
    <div class="bg-gray-100 flex justify-center items-center h-screen">
        <div class="w-400 h-800 flex flex-col justify-center items-center bg-white shadow-lg rounded-lg">
            <div class="animate-spin rounded-full h-32 w-32 border-t-4 border-b-4 border-blue-500"></div>
            <p class="mt-4 text-xl font-semibold text-gray-700">Loading...</p>
        </div>
    </div>
</template>

<script lang="ts">

</script>

<style scoped>
@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}
</style>