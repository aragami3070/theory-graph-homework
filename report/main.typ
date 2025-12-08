#import "conf.typ": conf, intro, conclusion
#show: conf.with(
  title: [Отчет по теории графов],
  type: "pract",
  info: (
    author: (
      name: [Смирнова Егора Ильича],
      faculty: [КНиИТ],
      group: "351",
      sex: "male",
    ),
    inspector: (
      degree: "доцент, к. ф.-м. н.",
      name: "С. В. Миронов",
    ),
  ),
  settings: (
    title_page: (
      enabled: true,),
    contents_page: (
      enabled: true,
    ),
  ),
)

//#intro
//#conclusion
#for value in ("01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11") {
  include "sections/" + value + ".typ"
}
