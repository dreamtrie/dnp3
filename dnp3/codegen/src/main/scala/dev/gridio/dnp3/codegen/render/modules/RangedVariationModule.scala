package dev.gridio.dnp3.codegen.render.modules

import dev.gridio.dnp3.codegen.model._
import dev.gridio.dnp3.codegen.model.groups.{Group10Var1, Group110AnyVar, Group1Var1, Group80Var1}
import dev.gridio.dnp3.codegen.render._

object RangedVariationModule extends Module {

  override def lines(implicit indent: Indentation) : Iterator[String] = {
      "use crate::app::variations::*;".eol ++
      "use crate::app::QualifierCode;".eol ++
      "use crate::app::parse::range::{RangedSequence, Range};".eol ++
      "use crate::util::cursor::ReadCursor;".eol ++
      "use crate::app::parse::parser::*;".eol ++
      "use crate::app::parse::bytes::*;".eol ++
      "use crate::app::parse::bit::{BitSequence, DoubleBitSequence};".eol ++
      "use crate::master::{ReadHandler, HeaderInfo};".eol ++
      "use crate::app::ObjectParseError;".eol ++
      space ++
      rangedVariationEnumDefinition ++
      space ++
      rangedVariationEnumImpl
  }

  private def rangedVariationEnumDefinition(implicit indent: Indentation) : Iterator[String] = {

    def getVarDefinition(v: Variation) : Iterator[String] = v match {
      case _ : SingleBitField => s"${v.name}(BitSequence<'a>),".eol
      case _ : DoubleBitField => s"${v.name}(DoubleBitSequence<'a>),".eol
      case _ : AnyVariation => s"${v.name},".eol
      case _ : FixedSize => s"${v.name}(RangedSequence<'a, ${v.name}>),".eol
      case _ : SizedByVariation if v.parent.groupType.isStatic =>  {
        s"${v.parent.name}Var0,".eol ++
        s"${v.parent.name}VarX(u8, RangedBytesSequence<'a>),".eol
      }
    }

    "#[derive(Debug, PartialEq)]".eol ++
      bracket("pub(crate) enum RangedVariation<'a>") {
        variations.iterator.flatMap(v =>  commented(v.fullDesc).eol ++ getVarDefinition(v))
      }

  }

  private def rangedVariationEnumImpl(implicit indent: Indentation) : Iterator[String] = {

    def getNonReadVarDefinition(v: Variation) : String = v match {
      case _ : AnyVariation => ""
      case _ : FixedSize => "(RangedSequence::parse(range, cursor)?)"
    }

    def getReadVarDefinition(v: Variation) : String = v match {
      case _ : AnyVariation => ""
      case _ : FixedSize => "(RangedSequence::empty())"
    }

    def getNonReadMatcher(v: Variation): Iterator[String] = v match {
      case _ : SingleBitField =>  {
        s"Variation::${v.name} => Ok(RangedVariation::${v.name}(BitSequence::parse(range, cursor)?)),".eol
      }

      case _ : DoubleBitField =>  {
        s"Variation::${v.name} => Ok(RangedVariation::${v.name}(DoubleBitSequence::parse(range, cursor)?)),".eol
      }

      case v : SizedByVariation => {
        s"Variation::${v.parent.name}(0) => Err(ObjectParseError::ZeroLengthOctetData),".eol ++
        bracketComma(s"Variation::${v.parent.name}(x) =>") {
          s"Ok(RangedVariation::${v.parent.name}VarX(x, RangedBytesSequence::parse(x, range.get_start(), range.get_count(), cursor)?))".eol
        }
      }
      case _ => s"Variation::${v.name} => Ok(RangedVariation::${v.name}${getNonReadVarDefinition(v)}),".eol
    }

    def getReadMatcher(v: Variation): Iterator[String] = v match {
      case _ : SingleBitField =>  {
        s"Variation::${v.name} => Ok(RangedVariation::${v.name}(BitSequence::empty())),".eol
      }

      case _ : DoubleBitField =>  {
        s"Variation::${v.name} => Ok(RangedVariation::${v.name}(DoubleBitSequence::empty())),".eol
      }
      case _ : SizedByVariation => {
        s"Variation::${v.parent.name}(0) => Ok(RangedVariation::${v.parent.name}Var0),".eol
      }
      case _ => s"Variation::${v.name} => Ok(RangedVariation::${v.name}${getReadVarDefinition(v)}),".eol
    }

    def getFmtMatcher(v: Variation): Iterator[String] = v match {
      case _ : AnyVariation => s"RangedVariation::${v.name} => Ok(()),".eol
      case _ : SizedByVariation => {
        s"RangedVariation::${v.parent.name}Var0 => Ok(()),".eol ++
          s"RangedVariation::${v.parent.name}VarX(_, seq) => format_indexed_items(f, seq.iter().map(|(x, i)| (Bytes::new(x), i))),".eol
      }
      case _ => s"RangedVariation::${v.name}(seq) => format_indexed_items(f, seq.iter()),".eol
    }

    def getMeasName(v: Variation): String = {
      v.parent.groupType match {
        case GroupType.StaticBinary => "binary_input"
        case GroupType.StaticDoubleBinary => "double_bit_binary_input"
        case GroupType.StaticBinaryOutputStatus => "binary_output_status"
        case GroupType.StaticAnalog => "analog_input"
        case GroupType.StaticAnalogOutputStatus => "analog_output_status"
        case GroupType.StaticCounter => "counter"
        case GroupType.StaticFrozenCounter => "frozen_counter"
        case _ => throw new Exception("unhandled variation")
      }
    }

    def getVariationMatcher(v: Variation): Iterator[String] = {
      v match {
        case _ : AnyVariation => {
          s"RangedVariation::${v.name} => Variation::${v.name},".eol
        }
        case _ : SizedByVariation => {
          s"RangedVariation::${v.parent.name}Var0 => Variation::${v.parent.name}(0),".eol ++
          s"RangedVariation::${v.parent.name}VarX(x, _) => Variation::${v.parent.name}(*x),".eol
        }
        case _ => {
          s"RangedVariation::${v.name}(_) => Variation::${v.name},".eol
        }
      }
    }


    def getExtractMatcher(v: Variation): Iterator[String] = {

      val isEvent = v.parent.groupType.isEvent;

      def simpleExtract(v: Variation): Iterator[String] = {
        bracket(s"RangedVariation::${v.name}(seq) =>") {
          parenSemi(s"handler.handle_${getMeasName(v)}") {
            s"HeaderInfo::new(self.variation(), qualifier, ${isEvent}, ${v.hasFlags}),".eol ++
            "&mut seq.iter().map(|(v,i)| (v.into(), i))".eol
          } ++ "true".eol
        }
      }

      v match {
        case _ : AnyVariation => {
          bracket(s"RangedVariation::${v.name} =>") {
              "false // qualifier 0x06".eol
          }
        }
        case Group80Var1 => {
          bracket(s"RangedVariation::${v.name}(_) =>") {
            "false // internal indications".eol
          }
        }
        case Group1Var1 => simpleExtract(v)
        case Group10Var1 => simpleExtract(v)
        case _ : DoubleBitField => simpleExtract(v)
        case _ : FixedSize => simpleExtract(v)
        case Group110AnyVar => {
          bracket(s"RangedVariation::${v.parent.name}Var0 =>") {
            "false".eol
          } ++
          bracket(s"RangedVariation::${v.parent.name}VarX(_,seq) =>") {
            parenSemi("handler.handle_octet_string") {
              s"HeaderInfo::new(self.variation(), qualifier, ${isEvent}, ${v.hasFlags}),".eol ++
              "&mut seq.iter()".eol
            } ++ "true".eol
          }
        }
      }
    }
    
    bracket("impl<'a> RangedVariation<'a>") {
      bracket("pub(crate) fn parse_non_read(v: Variation, qualifier: QualifierCode, range: Range, cursor: &mut ReadCursor<'a>) -> Result<RangedVariation<'a>, ObjectParseError>") {
        bracket("match v") {
          variations.flatMap(getNonReadMatcher).iterator ++ "_ => Err(ObjectParseError::InvalidQualifierForVariation(v, qualifier)),".eol
        }
      } ++ space ++
        bracket("pub(crate) fn parse_read(v: Variation, qualifier: QualifierCode) -> Result<RangedVariation<'a>, ObjectParseError>") {
          bracket("match v") {
            variations.flatMap(getReadMatcher).iterator ++ "_ => Err(ObjectParseError::InvalidQualifierForVariation(v, qualifier)),".eol
          }
        } ++ space ++
        bracket("pub(crate) fn format_objects(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result") {
          bracket("match self") {
            variations.flatMap(getFmtMatcher).iterator
          }
        } ++ space ++
        bracket("pub(crate) fn extract_measurements_to(&self, qualifier: QualifierCode, handler: &mut dyn ReadHandler) -> bool") {
          bracket("match self") {
            variations.flatMap(getExtractMatcher).iterator
          }
        } ++ space ++
        bracket("pub(crate) fn variation(&self) -> Variation") {
          bracket("match self") {
            variations.flatMap(getVariationMatcher).iterator
          }
        }
    }
  }

  def variations : List[Variation] = {
    ObjectGroup.allVariations.flatMap { v =>
      v match {
        case _ : DoubleBitField => Some(v)
        case _ : SingleBitField => Some(v)
        case v : AnyVariation if v.parent.groupType.isStatic => Some(v)
        case v : FixedSize if v.parent.groupType.isStatic => Some(v)
        case v : SizedByVariation if v.parent.groupType.isStatic => Some(v)
        case _ => None
      }
    }
  }

}
