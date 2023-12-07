use ue_types::*;

pub trait UObjectLike {
    fn object(&self) -> &UObject;
    fn name(&self) -> FName { self.object().name() }
    fn full_name(&self) -> String { self.object().full_name() }
}

impl UObjectLike for UActorComponent {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for USceneComponent {
    fn object(&self) -> &UObject { self.base_actor_component.object() }
}

impl UObjectLike for AActor {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for ACharacter {
    fn object(&self) -> &UObject { self.base_pawn.object() }
}

impl UObjectLike for APawn {
    fn object(&self) -> &UObject { self.base_actor.object() }
}

impl UObjectLike for UClass {
    fn object(&self) -> &UObject { &self.base_struct.object() }
}

impl UObjectLike for UEngine {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for UField {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for UGameEngine {
    fn object(&self) -> &UObject { self.base_engine.object() }
}

impl UObjectLike for UGameInstance {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for ULevel {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for ULocalPlayer {
    fn object(&self) -> &UObject { self.base_player.object() }
}

impl UObjectLike for UObject {
    fn object(&self) -> &UObject { self }
}

impl UObjectLike for UPlayer {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for UProperty {
    fn object(&self) -> &UObject { self.base_field.object() }
}

impl UObjectLike for UStruct {
    fn object(&self) -> &UObject { self.base_field.object() }
}

impl UObjectLike for UWorld {
    fn object(&self) -> &UObject { &self.base_object }
}

impl UObjectLike for AController {
    fn object(&self) -> &UObject { self.base_actor.object() }
}

impl UObjectLike for APlayerController {
    fn object(&self) -> &UObject { self.base_controller.object() }
}

impl UObjectLike for UConsole {
    fn object(&self) -> &UObject { &self.base_object }
}