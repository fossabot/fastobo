# coding: utf-8

import datetime
import unittest

import fastobo



class TestTypedPropertyValue(unittest.TestCase):

    type = fastobo.pv.TypedPropertyValue

    def test_init(self):
        rel = fastobo.id.UnprefixedIdent("creation_date")
        value = "2019-04-08T23:21:05Z"
        dt = fastobo.id.PrefixedIdent("xsd", "date")
        try:
            frame = self.type(rel, value, dt)
        except Exception:
            self.fail("could not create `TypedPropertyValue` instance")

    def test_init_type_error(self):
        rel = fastobo.id.UnprefixedIdent("creation_date")
        value = "2019-04-08T23:21:05Z"
        dt = fastobo.id.PrefixedIdent("xsd", "date")
        self.assertRaises(TypeError, self.type, 1, value, dt)
        self.assertRaises(TypeError, self.type, rel, 1, dt)
        self.assertRaises(TypeError, self.type, rel, value, 1)

    def test_property_relation(self):
        rel = fastobo.id.UnprefixedIdent("creation_date")
        value = "2019-04-08T23:21:05Z"
        dt = fastobo.id.PrefixedIdent("xsd", "date")
        pv = self.type(rel, value, dt)
        self.assertEqual(pv.relation, rel)

        rel2 = fastobo.id.PrefixedIdent("IAO", "0000219")
        pv.relation = rel2
        self.assertEqual(pv.relation, rel2)

        with self.assertRaises(TypeError):
            pv.relation = "IAO:0000219"

    def test_str(self):
        rel = fastobo.id.UnprefixedIdent("creation_date")
        value = "2019-04-08T23:21:05Z"
        dt = fastobo.id.PrefixedIdent("xsd", "date")
        pv = self.type(rel, value, dt)
        self.assertEqual(
            str(pv),
            'creation_date "2019-04-08T23:21:05Z" xsd:date'
        )

    def test_repr(self):
        rel = fastobo.id.UnprefixedIdent("creation_date")
        value = "2019-04-08T23:21:05Z"
        dt = fastobo.id.PrefixedIdent("xsd", "date")
        pv = self.type(rel, value, dt)
        self.assertEqual(
            repr(pv),
            "TypedPropertyValue("
            "UnprefixedIdent('creation_date'), "
            "'2019-04-08T23:21:05Z', "
            "PrefixedIdent('xsd', 'date'))"
        )
