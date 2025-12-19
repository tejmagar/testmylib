# README


## Update your settings.gradle.kts
```
	dependencyResolutionManagement {
		repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
		repositories {
			mavenCentral()
			maven { url 'https://jitpack.io' }
		}
	}
```


## Upgrade your build.gradle.ts (app module)

```

	dependencies {
	        implementation 'com.github.tejmagar:testmylib:0c83c8cc24'
	}
```
