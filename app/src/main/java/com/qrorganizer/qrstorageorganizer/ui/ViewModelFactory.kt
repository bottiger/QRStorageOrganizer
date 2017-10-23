package com.qrorganizer.qrstorageorganizer.ui

import android.arch.lifecycle.ViewModel
import android.arch.lifecycle.ViewModelProvider
import com.qrorganizer.qrstorageorganizer.persistance.ItemDao

/**
 * Factory for ViewModels
 */
class ViewModelFactory(private val dataSource: ItemDao) : ViewModelProvider.Factory {

    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(ItemViewModel::class.java)) {
            return ItemViewModel(dataSource) as T
        }
        throw IllegalArgumentException("Unknown ViewModel class")
    }
}