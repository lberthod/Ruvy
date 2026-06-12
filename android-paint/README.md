# Paint App - Android

Application Paint complète pour Android avec tous les outils de dessin!

## 🎨 Fonctionnalités

- ✏️ **Crayon** - Dessine librement
- 🧹 **Gomme** - Efface ce que tu as dessiné
- 📏 **Ligne** - Dessine des lignes droites
- 🎯 **Couleurs** - Sélecteur de couleur + 6 présets
- 📏 **Contrôles** - Taille et opacité ajustables
- ↶ **Annuler** - Reviens en arrière (50 étapes)
- 🗑️ **Effacer tout** - Nettoie le canvas
- ⬇️ **Télécharger** - Sauvegarde en PNG

## 📱 Compilation

### Prérequis
- **Android Studio** (télécharge-le depuis developer.android.com)
- **JDK 11+**
- **Android SDK** (inclus dans Android Studio)

### Méthode 1: Android Studio (Recommandé)

1. Ouvre Android Studio
2. Clique sur **File** → **Open**
3. Sélectionne le dossier `android-paint`
4. Android Studio téléchargera automatiquement les dépendances
5. Clique sur **Build** → **Build Bundle(s) / APK(s)** → **Build APK(s)**
6. L'APK sera créé dans `app/build/outputs/apk/debug/`

### Méthode 2: Ligne de commande

```bash
cd android-paint
./gradlew build
```

L'APK sera dans: `app/build/outputs/apk/debug/app-debug.apk`

### Méthode 3: Fichier APK signé (Pour Google Play)

```bash
./gradlew bundleRelease
```

## 📦 Installation

**Sur ton téléphone:**
1. Transfert du fichier APK sur ton téléphone
2. Active "Sources inconnues" dans **Paramètres** → **Sécurité**
3. Ouvre le fichier APK avec le gestionnaire de fichiers
4. Appuie sur **Installer**

## 🛠️ Structure du Projet

```
android-paint/
├── app/
│   ├── src/
│   │   └── main/
│   │       ├── java/com/ruvy/paint/
│   │       │   └── MainActivity.java
│   │       ├── res/
│   │       │   ├── layout/
│   │       │   ├── values/
│   │       │   ├── drawable/
│   │       │   └── xml/
│   │       ├── assets/
│   │       │   └── paint.html
│   │       └── AndroidManifest.xml
│   └── build.gradle
├── build.gradle
└── settings.gradle
```

## 🎯 Notes

- L'app embarque le HTML Paint directement
- Pas besoin de connexion Internet
- Compatible avec Android 5.0 (API 21) et plus
- Taille APK: ~3-5 MB

## 📝 Licence

Open source - Libre d'utilisation!

---

**Besoin d'aide?** Consulte la documentation Android: https://developer.android.com/
