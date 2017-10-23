package com.qrorganizer.qrstorageorganizer.persistance

import android.arch.persistence.room.Database
import android.arch.persistence.room.Room
import android.arch.persistence.room.RoomDatabase
import android.content.Context

/**
 * The Room database that contains the Users table
 */
@Database(entities = arrayOf(Item::class), version = 1)
abstract class ItemsDatabase : RoomDatabase() {

    abstract fun itemDao(): ItemDao

    companion object {

        @Volatile private var INSTANCE: ItemsDatabase? = null

        fun getInstance(context: Context): ItemsDatabase =
                INSTANCE ?: synchronized(this) {
                    INSTANCE ?: buildDatabase(context).also { INSTANCE = it }
                }

        private fun buildDatabase(context: Context) =
                Room.databaseBuilder(context.applicationContext,
                        ItemsDatabase::class.java, "Sample.db")
                        .build()
    }
}