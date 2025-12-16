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
      degree: "Старший преподаватель кафедры математики и программирования",
      name: "Портенко М.С.",
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

#for value in ("01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12") {
  include "sections/" + value + ".typ"
}
