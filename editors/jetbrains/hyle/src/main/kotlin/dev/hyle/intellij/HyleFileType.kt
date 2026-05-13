package dev.hyle.intellij

import com.intellij.openapi.fileTypes.LanguageFileType

class HyleFileType private constructor() : LanguageFileType(HyleLanguage) {
    override fun getName() = "Hyle"
    override fun getDescription() = "Hyle simulation language file"
    override fun getDefaultExtension() = "hyle"
    override fun getIcon() = HyleIcons.FILE

    companion object {
        @JvmField
        val INSTANCE = HyleFileType()
    }
}
