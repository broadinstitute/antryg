package antryg.util.conf

import java.nio.file.{Files, Path}
import java.util.Properties

case class PropertiesTree(valueOpt: Option[String], children: Map[String, PropertiesTree]) {

  import PropertiesTree.splitKeyPath

  def +(keyPath: String, value: String): PropertiesTree = this.+(splitKeyPath(keyPath), value)

  def +(keyPath: Seq[String], value: String): PropertiesTree =
    if (keyPath.isEmpty) {
      copy(valueOpt = Some(value))
    } else {
      val keyHead = keyPath.head
      val child = children.getOrElse(keyHead, PropertiesTree.empty)
      val childNew = child + (keyPath.tail, value)
      copy(children = children + (keyHead -> childNew))
    }

  def ++(valueMap: Map[String, String]) : PropertiesTree = {
    var treeNew: PropertiesTree = this
    for((keyPath, value) <- valueMap) {
      treeNew = treeNew + (keyPath, value)
    }
    treeNew
  }

  def get(keyPath: String): Option[String] = get(splitKeyPath(keyPath))

  def get(keyPath: Seq[String]): Option[String] =
    if(keyPath.isEmpty) {
      valueOpt
    } else {
      children.get(keyPath.head).flatMap(_.get(keyPath.tail))
    }

  def subTree(keyHead: String): PropertiesTree = children.getOrElse(keyHead, PropertiesTree.empty)
}

object PropertiesTree {
  val plainDotRegex: String = "\\."
  val empty: PropertiesTree = PropertiesTree(None, Map.empty)

  def splitKeyPath(keyPath: String): Seq[String] = {
    val keyPathTrimmed = keyPath.trim
    if (keyPathTrimmed.isEmpty) Seq.empty else keyPathTrimmed.split(plainDotRegex).toSeq
  }

  def fromProperties(properties: Properties): PropertiesTree = {
    import scala.collection.JavaConverters.propertiesAsScalaMapConverter
    var tree: PropertiesTree = empty
    for((key, value) <- properties.asScala) {
      tree = tree + (key, value)
    }
    tree
  }

  def fromFile(file: Path): PropertiesTree = {
    val properties = new Properties
    properties.load(Files.newInputStream(file))
    fromProperties(properties)
  }
}

