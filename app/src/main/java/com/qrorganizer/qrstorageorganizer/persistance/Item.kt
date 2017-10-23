package com.qrorganizer.qrstorageorganizer.persistance

/**
 * Created by bottiger on 23/10/2017.
 */
import android.arch.persistence.room.ColumnInfo
import android.arch.persistence.room.Entity
import android.arch.persistence.room.PrimaryKey
import java.util.*

@Entity(tableName = "items")
data class Item(@PrimaryKey
                @ColumnInfo(name = "itemid")
                val id: String = UUID.randomUUID().toString(),
                @ColumnInfo(name = "itemname")
                val itemName: String)