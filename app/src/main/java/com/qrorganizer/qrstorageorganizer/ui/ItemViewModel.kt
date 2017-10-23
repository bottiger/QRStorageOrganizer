package com.qrorganizer.qrstorageorganizer.ui

import android.arch.lifecycle.ViewModel
import com.qrorganizer.qrstorageorganizer.persistance.Item
import com.qrorganizer.qrstorageorganizer.persistance.ItemDao
import io.reactivex.Completable
import io.reactivex.Flowable
import io.reactivex.functions.Action
import io.reactivex.internal.operators.completable.CompletableFromAction

/**
 * View Model for the [UserActivity]
 */
class ItemViewModel(private val dataSource: ItemDao) : ViewModel() {

    /**
     * Get the user name of the user.
     * @return a [Flowable] that will emit every time the user name has been updated.
     */
    // for every emission of the user, get the user name
    fun userName(): Flowable<String> {
        return dataSource.getItemById(ITEM_ID)
                .map { item -> item.itemName }
    }

    /**
     * Update the user name.
     * @param userName the new user name
     * *
     * @return a [Completable] that completes when the user name is updated
     */
    fun updateUserName(itemName: String): Completable {
        return CompletableFromAction(Action {
            val item = Item(ITEM_ID, itemName)
            dataSource.insertUser(item)
        })
    }

    companion object {
        // using a hardcoded value for simplicity
        const val ITEM_ID = "1"
    }
}