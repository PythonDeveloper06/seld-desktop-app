<template>
  <Html>
    <Body>
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </Body>
  </Html>
</template>

<style lang="scss">
:root {
	--primary: #4aa3de;
	--primary-alt: #22c55e;
	--grey: #64748b;
	--dark: #1e293b;
	--dark-alt: #334155;
	--light: #f1f5f9;
	--sidebar-width: 200px;
}

* {
	margin: 0;
	padding: 0;
	box-sizing: border-box;
}

.app {
	display: flex;

	main {
		flex: 1 1 0;

		@media (max-width: 1024px) {
			padding-left: 3.0rem;
		}
	}
}
</style>
