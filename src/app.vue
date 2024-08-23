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
	--primary-alt: #2263c5;
	--grey: #64748b;
	--dark: #1e293b;
	--dark-alt: #334155;
	--light: #f1f5f9;
	--sidebar-width: 200px;
  --bg-footer: #0e0226;
}

* {
	margin: 0;
	padding: 0;
	box-sizing: border-box;
}

.app {
	display: flex;

	main {
		flex: 1 0 0;
    margin-left: 4rem;
	}
}

.box-device {
    padding: 20px;
    background: var(--dark);
    border-radius: 20px;
    border: 2px solid transparent;
    box-shadow: 0 0 5px var(--primary-alt);
    transition: all .50s ease;
    color: white;
}

.btn-device {
    border: 2px solid var(--primary);
    transition: all .5s ease;

    &:hover {
        background-color: var(--primary-alt);
    }
}
</style>
