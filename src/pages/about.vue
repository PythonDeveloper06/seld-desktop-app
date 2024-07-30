<script setup>

definePageMeta({
  layout: 'custom'
})

</script>

<template>

    <main>
        <div class="content">
            <div class="image">
                <img src="../assets/media/seld_pixian_ai.png" alt="None" class="text_logo">
            </div>
            <p>
                SELD - умный электронный замок контролируемый пользователем через приложение.
                Он сможет полностью настроить замок под свои нужды.
                Дистанционное открытие замка, создание постоянных и временных кодов доступа и NFC ключи,
                отслеживание состояние замка в реальном времени,
                ведение необходимой пользователю статистики и особые настройки безопасности
                (например изменение клавиатуры для большей защиты).
            </p>
        

            <div class="container">
                <h2 class="header">Мобильное приложение</h2>
                <p>
                    Приложение будет обладать удобным, красивым и минималистичным дизайном.
                    С помощью приложения пользователь сможет проводить настройку замка,
                    выставлять коды доступа и генерировать NFC ключи.
                    У пользователя может одновременно быть зарегистрировано несколько замков,
                    и пользователь сможет проводить индивидуальную настройку каждого замка.
                </p>
                <div class="text-center">Скачать мобильного приложение можно с нашего официального сайта.</div>
                <div class="all-btn">
                    <a class="a-btn rounded-lg" href="https://seld-lock.ru" target="_blank">Перейти на сайт</a>
                </div>
            </div>
        </div>

    </main>

</template>

<style scoped lang="scss">

.content {
    margin: 0 3rem;
}

.image {
    justify-content: center;
    align-items: center;
    display: flex;
    position: relative;
    margin-top: 1rem;
}

.image img {
    max-width: 350px;
    margin: auto;
}

.content p {
    font-size: 16px;
    text-align: justify;
}

main .header {
    margin-top: 1rem;
    color: var(--text-color);
    text-align: center;
    font-size: 2rem;
    font-weight: 600;
}

main p {
    text-align: justify;
    font-size: 16px;
    padding: 2px;
}

.all-btn {
    display: flex;
    justify-content: center;
}

.a-btn {
    display: inline-block;
    padding: 3px 10px;
    margin-top: 5px;
    background: transparent;
    border: 2px solid var(--primary);
    transition: all .5s ease;

    &:hover {
        background-color: var(--primary-alt);
        color: white;
    }
}

</style>