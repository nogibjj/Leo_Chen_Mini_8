"""Query the database"""

import sqlite3


def query():
    """CRUD"""
    conn = sqlite3.connect("US_births_DB.db")
    cursor = conn.cursor()

    # C(reate)
    cursor.execute(
        """
        INSERT INTO US_births_DB (year, month, date_of_month, day_of_week, births) 
        VALUES (2024, 10, 5, 6, 7785)"""
    )

    # R(ead)
    cursor.execute("SELECT * FROM US_births_DB")

    # U(pdate)
    cursor.execute(
        """
        UPDATE US_births_DB 
        SET births = 7899
        WHERE id = 55"""
    )

    # D(elete)
    cursor.execute(
        """
        DELETE FROM US_births_DB 
        WHERE id = 420"""
    )

    conn.commit()
    conn.close()
    return "Success!"