<script setup>

</script>

<template>
    <div class="grid header">
        <slot></slot>
    </div>
    <hr class="h-0.5 border-t-0 bg-neutral-400" />
</template>

<style scoped>

.header {
    background-color: var(--dark);
}

</style>