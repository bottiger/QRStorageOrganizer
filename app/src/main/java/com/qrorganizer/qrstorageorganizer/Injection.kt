package com.qrorganizer.qrstorageorganizer


import android.content.Context
import com.qrorganizer.qrstorageorganizer.persistance.ItemDao
import com.qrorganizer.qrstorageorganizer.persistance.ItemsDatabase
import com.qrorganizer.qrstorageorganizer.ui.ViewModelFactory

/**
 * Enables injection of data sources.
 */
object Injection {

    fun provideUserDataSource(context: Context): ItemDao {
        val database = ItemsDatabase.getInstance(context)
        return database.itemDao()
    }

    fun provideViewModelFactory(context: Context): ViewModelFactory {
        val dataSource = provideUserDataSource(context)
        return ViewModelFactory(dataSource)
    }
}