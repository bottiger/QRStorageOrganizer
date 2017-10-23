package com.qrorganizer.qrstorageorganizer.persistance

/**
 * Created by bottiger on 23/10/2017.
 */
import android.arch.persistence.room.Dao
import android.arch.persistence.room.Insert
import android.arch.persistence.room.OnConflictStrategy
import android.arch.persistence.room.Query

import io.reactivex.Flowable

/**
 * Data Access Object for the users table.
 */
@Dao
interface ItemDao {

    /**
     * Get a user by id.
     * @return the user from the table with a specific id.
     */
    @Query("SELECT * FROM Items WHERE itemid = :id")
    fun getItemById(id: String): Flowable<Item>

    /**
     * Insert a user in the database. If the user already exists, replace it.
     * @param user the user to be inserted.
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    fun insertUser(item: Item)

    /**
     * Delete all users.
     */
    @Query("DELETE FROM Items")
    fun deleteAllItems()
}