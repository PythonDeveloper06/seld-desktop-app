<script setup>

definePageMeta({
  layout: 'custom'
})

</script>
<template>
    <main>
        <div class="about_text">
            <div class="box">
                <div class="for_img">
                    <img src="../assets/media/Danya.jpg" class="rounded_image">
                </div>
                <p>Полуян Даниил Артемович, абитуриент. 
                    Разработка софта для устройства и его прошивка 
                </p>
            </div>
            <div class="box">
                <div class="for_img">
                    <img src="../assets/media/Pasha.jpg" class="rounded_image">
                </div>
                <p>Леднев Павел Денисович, абитуриент. 
                    Создание и поддержка сайта и API для обмена данными между устройствами, 
                    а также создание настольного приложения под разные ОС
                </p>
            </div>
            <div class="box">
                <div class="for_img">
                    <img src="../assets/media/Sanya.jpg" class="rounded_image">
                </div>
                <p>Рящиков Александр Григорьевич, абитуриент. 
                    3D моделирование, разработка макета устройства, экономический анализ, менеджмент
                </p>
            </div>
            <div class="box">
                <div class="for_img">
                    <img src="../assets/media/Misha.jpg" class="rounded_image">
                </div>
                <p>Гордюшин Михаил Дмитриевич, абитуриент. 
                    Создание и поддержка приложения, дизайн приложения
                </p>
            </div>
        </div>
    </main>
        
</template>

<style scoped>

.about_text {
    margin: 2rem 3rem;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1.1rem;
}

.box {
    padding: 70px 30px 40px;
    background: var(--dark);
    border-radius: 30px;
    border: 2px solid transparent;
    box-shadow: 0 0 5px var(--primary-alt);
    transition: all .50s ease;
    cursor: pointer;
    color: white;
}

.rounded_image {
    border-radius: 50%;
    width: 150px;
    border: 5px solid var(--primary-alt);
    box-shadow: 0 0 20px var(--primary-alt);
    margin: -2rem 0 1rem 0;
}

.for_img {
    display: flex;
    justify-content: center;
    align-items: center;
}

@media(max-width: 1250px) {
    .about_text {
        margin: 2rem 1rem;
    }
}

@media(max-width: 1150px) {
    .about_text {
        margin: 2rem 2rem;
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 1.25rem;
    }
}

@media(max-width: 700px) {
    .about_text {
        grid-template-columns: repeat(1, 1fr);
    }
}

</style>